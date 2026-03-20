# Configuración SSL/mTLS — Mi Cronómetro PSP

**Última actualización**: 25 febrero 2026
**Estado**: ✅ Operativo en producción

---

## 1. Visión general

El acceso externo a la aplicación usa **TLS mutuo (mTLS)**:

- El servidor presenta su certificado (Let's Encrypt, confiado por los navegadores).
- El cliente debe presentar un certificado de cliente firmado por nuestra CA privada.
- Sin certificado de cliente → Apache rechaza la conexión con 403 (antes de llegar al PHP).

Esto actúa como capa de autenticación sin contraseña, ligada al dispositivo.

```
Móvil/PC exterior
  │
  │  HTTPS :443  (TLS 1.2+, ECC)
  ▼
Router MitraStar
  │  Port forward 443 → 192.168.1.71:443
  ▼
Apache (NAS WD EX2 Ultra)
  │  SSLVerifyClient require  ← rechaza si no hay cert de cliente
  │  SSLCACertificateFile = ca.crt
  ▼
Aplicación PHP / frontend estático
```

---

## 2. Archivos: dónde está todo

Todos los archivos SSL viven en disco persistente (sobreviven reinicios):

```
/mnt/HD/HD_a2/.cronometro-psp/ssl/
├── ca.crt          ← Certificado de la CA (público, se da a los clientes)
├── ca.key          ← Clave privada de la CA  ⚠️ CRÍTICA — solo en el NAS
├── ca.srl          ← Contador de serie para certificados emitidos
├── client.crt      ← Certificado de cliente "cpcxb-movil" (instalado en dispositivos)
├── client.csr      ← CSR original (conservado por documentación)
├── client.key      ← Clave privada del cliente
├── client.p12      ← Bundle PKCS#12 (cert + clave) para instalar en iOS/Android
├── fullchain.pem   ← Certificado de servidor Let's Encrypt (cadena completa)
└── privkey.pem     ← Clave privada del certificado de servidor
```

---

## 3. CA privada (Cronometro-CA)

### 3.1 Metadatos

| Campo | Valor |
|-------|-------|
| Subject | `C=ES, O=Mi Cronometro PSP, CN=Cronometro-CA` |
| Válido desde | 2026-02-20 |
| Válido hasta | 2036-02-18 (10 años) |
| SHA1 Fingerprint | `C4:E3:15:33:72:7D:A6:B3:D8:76:56:E3:F5:19:4E:E0:F8:58:AA:40` |

### 3.2 Cómo se creó (para reproducir si fuera necesario)

```bash
# En el NAS, vía SSH:
cd /mnt/HD/HD_a2/.cronometro-psp/ssl/

# 1. Generar clave de la CA (ECC P-384)
openssl ecparam -name secp384r1 -genkey -noout -out ca.key

# 2. Generar certificado auto-firmado de la CA (10 años)
openssl req -new -x509 -days 3650 -key ca.key -out ca.crt \
    -subj "/C=ES/O=Mi Cronometro PSP/CN=Cronometro-CA"
```

---

## 4. Certificados de cliente

### 4.1 Dispositivos con certificado instalado

| CN | Fichero p12 | Instalado en |
|----|-------------|--------------|
| `cpcxb-movil` | `client.p12` | Móvil principal (Feb 2026) |

Válido: 2026-02-20 → 2036-02-18 (10 años).

### 4.2 Cómo añadir un nuevo dispositivo

```bash
# En el NAS, vía SSH:
cd /mnt/HD/HD_a2/.cronometro-psp/ssl/
NOMBRE="nuevo-dispositivo"   # sin espacios, ej: ipad-trabajo

# 1. Generar clave privada del cliente
openssl ecparam -name prime256v1 -genkey -noout -out ${NOMBRE}.key

# 2. Crear CSR
openssl req -new -key ${NOMBRE}.key -out ${NOMBRE}.csr \
    -subj "/C=ES/O=Mi Cronometro PSP/CN=${NOMBRE}"

# 3. Firmar con la CA (10 años)
openssl x509 -req -days 3650 \
    -in ${NOMBRE}.csr \
    -CA ca.crt -CAkey ca.key -CAserial ca.srl \
    -out ${NOMBRE}.crt

# 4. Empaquetar en PKCS#12 para instalar en el dispositivo
# (pedirá una contraseña de exportación — puede dejarse vacía)
openssl pkcs12 -export \
    -in ${NOMBRE}.crt \
    -inkey ${NOMBRE}.key \
    -certfile ca.crt \
    -out ${NOMBRE}.p12

# 5. Copiar el .p12 al PC para instalarlo en el dispositivo
# Desde WSL/PC:
scp -i /tmp/id_nas_deploy \
    sshd@192.168.1.71:/mnt/HD/HD_a2/.cronometro-psp/ssl/${NOMBRE}.p12 \
    ~/Downloads/
```

**Instalar el `.p12` en el dispositivo:**
- **iOS/iPadOS**: AirDrop el `.p12` → Ajustes → General → VPN y gestión del dispositivo → instalar perfil
- **Android**: Ajustes → Seguridad → Credenciales → Instalar desde almacenamiento
- **Windows**: doble clic en el `.p12` → Asistente de importación → almacén "Personal"
- **macOS**: doble clic → llavero "Inicio de sesión"

---

## 5. Certificado de servidor (Let's Encrypt)

### 5.1 Metadatos actuales

| Campo | Valor |
|-------|-------|
| Dominio | `cronometro.hash-pointer.com` |
| Emisor | Let's Encrypt E7 |
| Válido hasta | 2026-05-20 |
| Próxima renovación | ~2026-03-20 (automática) |
| Algoritmo | ECDSA P-256 (ec-256) |

### 5.2 Herramienta: acme.sh

Instalado en `/mnt/HD/HD_a2/.cronometro-psp/.acme.sh/`.
Se ejecuta automáticamente cada 5 min desde `autorestaurar.sh` (paso 12).
Usa DNS-01 challenge vía GoDaddy API (`dns_gdfix`) — no requiere abrir el puerto 80.

**Claves GoDaddy** (NO incluidas aquí por seguridad):
- Están en `ddns-update.sh` y en `.acme.sh/account.conf` (ambos en el NAS).

**Renovar manualmente** (en caso de urgencia):
```bash
ssh -i ~/.ssh/id_nas sshd@192.168.1.71
ACME=/mnt/HD/HD_a2/.cronometro-psp/.acme.sh/acme.sh
$ACME --renew -d cronometro.hash-pointer.com --force --home $(dirname $ACME)
```

---

## 6. Configuración Apache

### 6.1 VirtualHost HTTPS (:443) — `ssl-cronometro.conf`

Ruta en disco: `/mnt/HD/HD_a2/.cronometro-psp/ssl-cronometro.conf`
Cargado en: `/usr/local/apache2/conf/extra/ssl-cronometro.conf`

Directivas clave de mTLS:
```apache
SSLCACertificateFile /mnt/HD/HD_a2/.cronometro-psp/ssl/ca.crt
SSLVerifyClient require
SSLVerifyDepth 1
```

### 6.2 VirtualHost HTTP (:8080) — `http-cronometro.conf`

Solo acepta conexiones desde `192.168.1.x` o `127.0.0.1`. Sin mTLS en LAN.

---

## 7. DDNS (GoDaddy)

Script: `/mnt/HD/HD_a2/.cronometro-psp/ddns-update.sh`
Se ejecuta cada 5 min desde `autorestaurar.sh` (paso 11).

Flujo:
1. Obtiene IP pública vía `api.ipify.org` (fallback: `ifconfig.me`)
2. Compara con la última IP guardada en `/tmp/cronometro-last-ip.txt`
3. Si cambió, actualiza el registro A de `cronometro.hash-pointer.com` vía GoDaddy REST API

---

## 8. Recuperación tras reinicio

`autorestaurar.sh` restaura automáticamente (vía crontab `*/5 * * * *`):

| Paso | Qué restaura |
|------|-------------|
| 1 | `authorized_keys` SSH |
| 2 | `Listen 8080` en `ports.conf` |
| 3 | Bloques `Directory` en `httpd.conf` |
| 5 | Archivos de la app (frontend + backend) |
| 10 | `ssl-cronometro.conf`, `http-cronometro.conf`, `mod_ssl` |
| 11 | DDNS (actualiza IP si cambió) |
| 12 | Renovación acme.sh (comprueba si el cert expira en <30 días) |

> ⚠️ **Bug conocido en `autorestaurar.sh` (paso 10)**:
> Las variables `SSL_CONF_SRC` y `HTTP_CONF_SRC` apuntan a `/ssl-cronometro.conf` y
> `/http-cronometro.conf` (raíz del sistema de archivos) en lugar de
> `$DISK_DIR/ssl-cronometro.conf`. Si esos archivos no existen en `/`, la restauración
> de los VirtualHost SSL/HTTP fallará silenciosamente tras un reinicio.
> **Acción pendiente**: corregir las dos variables en `autorestaurar.sh`.

---

## 9. Revocar un certificado de cliente

La CA no tiene CRL configurada. Para revocar el acceso de un dispositivo:

1. **Opción simple**: regenerar la CA completa y emitir nuevos certificados solo para los dispositivos autorizados. Como la CA caduca en 2036, esto es práctico.
2. **Opción avanzada**: configurar `SSLCARevocationFile` con un CRL generado con `openssl ca -gencrl`.

---

## 10. Verificar que mTLS funciona

Desde WSL o cualquier terminal con openssl:

```bash
# Debe responder 200 (con el cert de cliente)
curl -v \
  --cert /ruta/al/client.crt \
  --key  /ruta/al/client.key \
  https://cronometro.hash-pointer.com/apps/cronometro/api/api/health

# Debe ser rechazado (sin cert de cliente)
curl -v https://cronometro.hash-pointer.com/apps/cronometro/api/api/health
# → expected: SSL handshake error / 400 Bad Request
```
