# EMERGENCIA DE RED — 23/02/2026

## Situación al reiniciar

- **NAS (192.168.1.71)**: inaccesible desde este PC, pero accesible desde otros equipos → el NAS está bien
- **Este PC (Windows)**: tiene IP `192.168.1.66` en Ethernet, pero no llega a .71
- **WSL2**: en red NAT interna (172.28.x.x), sin ruta hacia 192.168.1.x

## Diagnóstico

```
Ethernet     → 192.168.1.66   ✅ (tiene IP en la LAN)
Wi-Fi        → 169.254.x.x    ❌ (APIPA, sin DHCP)
vEthernet WSL→ 172.28.176.1   (adaptador virtual Hyper-V)
```

El ping a 192.168.1.71 responde desde 192.168.1.66 con "Host de destino inaccesible",
lo que indica que el equipo Windows sabe enrutar hacia .71 pero la capa de red
(ARP / switch / VLAN) no está respondiendo. Posible causa: el adaptador Ethernet
de Windows perdió la conexión física o el switch necesita reiniciarse.

## Lo primero tras el reinicio

1. Verificar que el cable Ethernet está bien conectado (en PC y en switch/router)
2. Desde CMD de Windows: `ping 192.168.1.71`
3. Si falla: `ping 192.168.1.1` (router) — si también falla, el problema es el adaptador o el cable
4. Si el ping al router funciona pero no al NAS: el NAS puede haber cambiado de IP por DHCP

## Si el NAS cambió de IP

Buscar el NAS en la LAN:
```
# PowerShell
1..254 | ForEach-Object { $ip="192.168.1.$_"; if(Test-Connection $ip -Count 1 -Quiet -TimeoutSeconds 1){ Write-Host "$ip responde" }}
```

O acceder al panel del router para ver los dispositivos conectados y buscar "WDMyCloud" o la MAC del NAS.

## Para arreglar WSL2 (después de recuperar Windows)

```bash
# En terminal WSL:
sudo ip route add 192.168.1.0/24 via 172.28.176.1
```

Para hacerlo permanente, añadir a `/etc/wsl.conf` o a un script de inicio de WSL.

---

## Estado del proyecto al parar

### Git — commits pendientes de ejecutar tras recuperar red
Los archivos de tests están creados localmente pero NO commiteados aún.
Archivos nuevos sin commitear:
- `tests/php/bootstrap.php`
- `tests/php/phpunit.xml`
- `tests/php/SesionesTest.php`
- `tests/php/fixtures/test_schema.sql`
- `tests/js/runner.js`
- `tests/js/app.functions.test.js`
- `tests/smoke/smoke-test.sh`
- `tests/README.md`

### Tarea pendiente: ejecutar y verificar los tests
1. **Instalar PHP CLI en WSL** (necesita red o apt cache):
   ```bash
   sudo apt-get install -y php-cli php-sqlite3
   wget https://phar.phpunit.de/phpunit-11.phar -O tests/php/phpunit.phar
   chmod +x tests/php/phpunit.phar
   ```

2. **Ejecutar tests PHP**:
   ```bash
   php tests/php/phpunit.phar --configuration tests/php/phpunit.xml
   ```

3. **Ejecutar tests JS** (solo necesita Node, sin red):
   ```bash
   node tests/js/runner.js
   ```

4. **Ejecutar smoke tests** (necesita NAS accesible):
   ```bash
   bash tests/smoke/smoke-test.sh
   ```

5. **Commitear todo**:
   ```bash
   git add tests/
   git commit -m "feat: suite de tests (PHPUnit + Node + smoke bash)"
   ```

6. **Investigar licencias** (pendiente):
   - PHPUnit: BSD-3-Clause
   - Material Symbols: Apache 2.0
   - Node.js: MIT
   - PHP: PHP License v3.01
   - SQLite: dominio público
   — Confirmar y documentar en un archivo LICENSES.md o en el README del proyecto

### Versiones en producción (NAS)
- `styles.css` v7, `api-client.js` v6, `app.js` v14
- Último commit: `2c7d6b3` (docs: CHANGELOG v1.2.0)
- El código de tests NO está en el NAS (solo en local), no requiere deploy

### URL de acceso al NAS
- LAN: http://192.168.1.71:8080/apps/cronometro/www/
- Externo: https://cronometro.hash-pointer.com/

### Script de recuperación del NAS (si se reinicia)
```bash
bash scripts/recuperar-nas.sh
```
