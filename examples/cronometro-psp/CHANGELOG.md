# Changelog - Mi Cronómetro PSP

Todos los cambios notables se documentan en este archivo.
Formato basado en [Keep a Changelog](https://keepachangelog.com/es/1.0.0/).

---

## [1.2.0] - 2026-02-23

### Añadido
- Opción "Sustituye a la tarea en curso" (checkbox en modal de inicio): actualiza `tarea_id` y notas de la sesión activa sin crear fila nueva; solo visible cuando hay sesión en marcha
- Toast de aviso (no bloqueante, 4s) cuando el inicio retroactivo se recortó por limitación de la sesión anterior
- Garantía de duración mínima 1 min para la sesión anterior al usar retroactivo

### Mejorado
- Retroactivo con sesión previa activa: ahora funciona correctamente. La sesión anterior se cierra en el instante de inicio de la nueva (no en `$ahora`), permitiendo retroactivos reales. El límite es `inicio_sesion_anterior + 60s`
- Versiones: `styles.css` v7, `api-client.js` v6, `app.js` v14

---

## [1.1.0] - 2026-02-21

### Añadido
- Modo edición con botón ✏️ junto al botón + (zona inferior derecha)
  - En modo normal: clic activa la tarea sin delay
  - En modo edición: fondo rosado + clic abre el editor de tarea
- Repositorio Git local inicializado con tag `v1.0` en el estado previo a esta versión

### Mejorado
- Grid de tareas: 4 columnas fijas en cualquier pantalla (antes 3 en móvil)
- Tarjetas más compactas (texto 0.72rem, min-height 80px) manteniendo el tamaño del icono
- Pestañas de actividades con `flex-wrap` (varias filas si hay muchas actividades)
- Táctil: umbral de movimiento de 10px para cancelar tap (evita activaciones al desplazarse)

### Corregido
- `detenerSesion()` no incluía `tarea_id` en el SELECT inicial → `usos_7d` nunca se incrementaba al detener manualmente
- Botón ✏️ ya no solapa el timer ni el botón ⚙️ al estar en la zona inferior

### Eliminado
- Lógica de doble clic con delay 280ms en PC
- Long-press táctil (600ms) para abrir editor

---

## [1.0.0] - 2026-02-19 — MVP ✅

### Funcionalidad core
- Timer de sesión activa en formato HH:MM (resolución de minutos)
- Tiempo acumulado por tarea y total del día en barra inferior ("Hoy: H:MM")
- Pestañas de actividades generadas dinámicamente desde la BD (no hardcodeadas)
- Pestaña "⭐ Frecuentes" con top 8 tareas de `usos_7d > 0`; vacía tras puesta a cero
- Comentario opcional al iniciar tarea (guardado en BD, visible en barra inferior)
- Modal de selección de actividad para tareas multi-actividad

### Gestión de actividades y tareas
- Crear actividades con nombre, color y flag permanente (🔒 Conservar tras puesta a cero)
- Crear tipos de tarea con emoji, nombre y actividades permitidas
- IDs generados automáticamente como slugs del nombre

### Configuración
- Menú ⚙️ con: "Nueva actividad", "Acerca de", "Puesta a cero"
- **Acerca de**: estado de conexión con latencia + tiempo acumulado histórico por actividad
- **Puesta a cero** (3 fases):
  1. Aviso de lo que ocurrirá
  2. Lista de actividades con checkboxes (permanentes pre-marcadas con 🔒)
  3. Confirmación escribiendo "BORRAR"
- Tras la puesta a cero: CSV exportado, sesiones borradas, `usos_7d` reseteado, actividades no marcadas archivadas, tipos de tarea huérfanos archivados

### Backend / Infraestructura
- Backend PHP + SQLite en NAS WD My Cloud EX2 Ultra (192.168.1.71:8080)
- Compensación de offset de reloj cliente/servidor via `server_time` en API
- Migración al vuelo de columnas BD (`archived` en tipos_tarea, `permanente` en actividades)
- Scripts de deploy (`deploy-nas.sh`) y recuperación (`recuperar-nas.sh`) sin contraseña
- `autorestaurar.sh` en disco persistente: restaura Apache, cron y sincronización NTP tras reinicios

### Corrección de bugs
- Timer arrancaba en negativo (reloj NAS adelantado ~61s) → compensación con `clockOffset`
- Timer mostraba segundos; reducido a minutos con truncado al minuto
- `usos_7d` en PDO devuelto como string → comparación laxa `> 0` en JS
- `duracion` en PDO devuelto como string → `Number()` al sumar
- Caché del móvil no descargaba JS actualizado → `?v=N` en src de scripts
- Frecuentes mostraba 8 tareas con 0:00 tras puesta a cero → filtro `usos_7d > 0`
- Pestañas de actividades no desaparecían tras puesta a cero → archivado real en BD + regeneración dinámica de UI

---

## [0.2.0] - 2026-02-18

### Añadido
- Sincronización NTP via `sntp` en cron cada 5 minutos (no hay ntpd en NAS WD)
- Clave SSH almacenada en Windows (`C:\Users\cpcxb\.ssh\id_nas`) para persistencia entre sesiones WSL
- Scripts de deploy y recuperación completos y automatizados

### Corregido
- Reloj del NAS divergía respecto a NTP a pesar de la configuración en la UI del NAS (ntpd inactivo)
- Petición de contraseña SSH en cada ejecución de scripts (clave efímera en WSL)

---

## [0.1.0] - 2026-02-17

### Añadido
- Arquitectura inicial: PHP + SQLite en NAS, frontend estático en Apache :8080
- Endpoints: actividades, tipos-tarea, sesiones (iniciar, detener, listar, activa)
- Timer básico con sesión activa
- Grid de tareas con pestañas hardcodeadas (5 actividades iniciales)
- Datos iniciales: Proyecto X, Proyecto Y, Admin, No productivo, Personal + 16 tipos de tarea

---

*Para ideas y funcionalidades futuras ver `docs/futuras-versiones.md`*
