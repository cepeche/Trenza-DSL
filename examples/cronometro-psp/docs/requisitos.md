# Mi Cronómetro PSP - Especificación de Requisitos
**Versión**: 1.2
**Fecha**: 23 febrero 2026
**Autor**: César (Ingeniero de Caminos)
**Actualizado por**: Revisión v1.1 + propuestas de mejora de Claude

---

## 1. Introducción

### 1.1 Propósito
Este documento especifica los requisitos funcionales y no funcionales para "Mi Cronómetro PSP", una aplicación web de seguimiento de tiempo dedicado a actividades, con tracking del progreso de tareas.

### 1.2 Ámbito
- **Nombre del sistema**: Mi Cronómetro Personal Software Process
- **Usuarios objetivo**: Profesionales que desean seguimiento personal de tiempo y productividad
- **Plataforma inicial**: Web (Chrome/Firefox en desktop y móvil)
- **Backend**: PHP + SQLite en NAS WD My Cloud EX2 Ultra (192.168.1.71:8080)
- **Frontend**: HTML/CSS/JS estático servido por Apache

### 1.3 Definiciones
- **Actividad**: Proyecto o categoría de trabajo (ej: "Proyecto X", "Admin", "No productivo")
- **Tipo de Tarea**: Tipo de trabajo que puede realizarse en una o más actividades (ej: "Codificar", "Reuniones")
- **Tarea**: Instancia de un tipo de tarea vinculada a una actividad específica (ej: "Codificar en Proyecto X")
- **Sesión**: Periodo continuo de tiempo dedicado a una tarea específica
- **Sesión activa**: La sesión que está en curso actualmente
- **Actividad permanente**: Actividad marcada para conservarse tras una puesta a cero

---

## 2. Requisitos Funcionales

### RF-001: Visualizar actividades y tipos de tarea
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El sistema debe mostrar los tipos de tarea disponibles organizados en pestañas por actividad.

**Criterios de aceptación**:
- CA-001.1: Se muestra una pestaña "⭐ Frecuentes" con los tipos de tarea más usados
- CA-001.2: Se muestran pestañas adicionales para cada actividad existente, generadas dinámicamente desde la BD
- CA-001.3: Los tipos de tarea se muestran como tarjetas con icono, nombre y tiempo acumulado del día
- CA-001.4: Se muestran 4 tarjetas por fila en cualquier pantalla
- CA-001.5: Las pestañas de actividades desaparecen si la actividad se archiva
- CA-001.6: Las pestañas forman varias filas si no caben en una sola (flex-wrap)

**Relacionado con**: RF-002, RF-003, RF-010

---

### RF-002: Iniciar sesión de trabajo
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El usuario debe poder iniciar una sesión de trabajo pulsando sobre un tipo de tarea.

**Precondiciones**:
- Existen tipos de tarea configurados en el sistema

**Flujo normal**:
1. Usuario pulsa sobre tarjeta de tipo de tarea
2. Sistema muestra modal de comentario opcional
3. Usuario escribe comentario (o lo deja vacío) y pulsa "Iniciar"
4. Si el tipo de tarea solo puede usarse en una actividad:
   - Sistema inicia sesión inmediatamente para esa tarea
5. Si el tipo de tarea puede usarse en múltiples actividades:
   - Sistema muestra modal de selección de actividad
   - Usuario selecciona actividad
   - Sistema inicia sesión para la tarea en esa actividad
6. Si había sesión activa previa:
   - Sistema cierra sesión anterior calculando duración
   - Sistema guarda sesión cerrada en historial
7. Sistema crea nueva sesión activa con timestamp de inicio
8. Sistema actualiza visualización del timer

**Criterios de aceptación**:
- CA-002.1: Al pulsar tipo de tarea unívoca, se abre modal de comentario directamente
- CA-002.2: Al pulsar tipo de tarea multi-actividad, se muestra modal de actividades primero, luego comentario
- CA-002.3: La sesión anterior se cierra automáticamente al iniciar nueva sesión
- CA-002.4: La duración de sesión cerrada se calcula como: tiempo_fin - tiempo_inicio
- CA-002.5: El timer muestra la nueva tarea activa inmediatamente
- CA-002.6: El comentario (si se escribe) se muestra en la barra inferior izquierda

**Relacionado con**: RF-003, RF-004, RF-011, RF-013

---

### RF-003: Visualizar sesión activa
**Prioridad**: Alta
**Estado**: ✅ Implementado (resolución de minutos)
**Descripción**: El sistema debe mostrar en todo momento la sesión de trabajo activa.

**Criterios de aceptación**:
- CA-003.1: El header superior muestra: icono (renderizado) + nombre de tarea + actividad
- CA-003.2: El timer muestra duración en formato HH:MM (resolución de minutos)
- CA-003.3: El timer se actualiza cada minuto
- CA-003.4: Si no hay sesión activa, muestra "Ninguna tarea activa" y "--:--"
- CA-003.5: El header es sticky (visible al hacer scroll)
- CA-003.6: La tarjeta de la tarea activa se destaca visualmente (borde, fondo)
- CA-003.7: El tiempo mostrado compensa el offset de reloj entre cliente y servidor

**Relacionado con**: RF-002, RF-012

---

### RF-004: Calcular tiempo acumulado por tipo de tarea
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El sistema debe mostrar el tiempo total dedicado a cada tipo de tarea en el día actual.

**Criterios de aceptación**:
- CA-004.1: Cada tarjeta muestra tiempo acumulado HOY para ese tipo de tarea
- CA-004.2: El tiempo incluye todas las sesiones cerradas del tipo de tarea
- CA-004.3: Si el tipo de tarea está activo, se suma el tiempo transcurrido actual
- CA-004.4: El tiempo se actualiza cada minuto si la tarea está activa
- CA-004.5: Formato de tiempo: "H:MM" (ej: "1:30", "0:45")

**Relacionado con**: RF-002, RF-012

---

### RF-005: Calcular tiempo total del día
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El sistema debe mostrar el tiempo total trabajado en el día actual.

**Criterios de aceptación**:
- CA-005.1: Barra inferior derecha muestra "Hoy: H:MM"
- CA-005.2: Suma todas las sesiones cerradas del día
- CA-005.3: Suma el tiempo de la sesión activa si existe
- CA-005.4: Se actualiza cada minuto
- CA-005.5: El valor se resetea a medianoche (basado en timestamps del servidor)

**Relacionado con**: RF-004, RF-012

---

### RF-006: Pestaña "Frecuentes"
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: El sistema debe mostrar una pestaña con los tipos de tarea más utilizados recientemente.

**Criterios de aceptación**:
- CA-006.1: Se muestran los tipos de tarea con `usos_7d > 0`, ordenados por frecuencia, máximo 8
- CA-006.2: Se añaden los 3 tipos de tarea más recientes del día (sin duplicar)
- CA-006.3: Si `usos_7d = 0` para todos (ej: tras puesta a cero), la pestaña queda vacía
- CA-006.4: Esta pestaña es la primera (más a la izquierda)
- CA-006.5: Es la pestaña activa por defecto al cargar la aplicación

**Relacionado con**: RF-001, RF-010

---

### RF-007: Crear nuevo tipo de tarea
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: El usuario debe poder crear nuevos tipos de tarea desde la interfaz.

**Flujo normal**:
1. Usuario pulsa botón flotante "+"
2. Sistema muestra modal "Nueva tarea"
3. Usuario introduce nombre, selecciona icono (emoji), marca actividades
4. Usuario pulsa "Crear"
5. Sistema valida nombre no vacío y al menos una actividad
6. Sistema crea tipo de tarea y tareas expandidas (una por actividad)
7. Sistema actualiza visualización

**Criterios de aceptación**:
- CA-007.1: Modal muestra campo de texto para nombre
- CA-007.2: Modal muestra grid de 25+ emojis predefinidos
- CA-007.3: Modal muestra checkboxes de todas las actividades existentes
- CA-007.4: Si modal se abre desde pestaña de actividad, esa actividad viene pre-seleccionada
- CA-007.5: Tras crear, la nueva tarea aparece en las pestañas correspondientes
- CA-007.6: ID del tipo de tarea se genera automáticamente (slug del nombre)

**Relacionado con**: RF-008, RF-009

---

### RF-008: Crear nueva actividad
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: El usuario debe poder crear nuevas actividades desde la configuración.

**Flujo normal**:
1. Usuario accede a menú de configuración (⚙️) → "Nueva actividad"
2. Sistema muestra modal con campos: nombre, color, toggle permanente
3. Usuario rellena y pulsa "Crear"
4. Sistema crea actividad y pestaña correspondiente

**Criterios de aceptación**:
- CA-008.1: Modal muestra campo de texto para nombre
- CA-008.2: Modal muestra grid de 10 colores predefinidos
- CA-008.3: Modal muestra toggle "🔒 Conservar tras puesta a cero" (campo `permanente`)
- CA-008.4: ID de actividad se genera automáticamente (slug del nombre)
- CA-008.5: Nueva pestaña aparece en el listado de pestañas inmediatamente

**Relacionado con**: RF-007, RF-009, RF-015

---

### RF-009: Menú de configuración
**Prioridad**: Baja
**Estado**: ✅ Implementado
**Descripción**: El sistema debe proporcionar acceso a funciones de configuración.

**Criterios de aceptación**:
- CA-009.1: Botón ⚙️ visible en header del timer
- CA-009.2: Al pulsar, despliega menú dropdown
- CA-009.3: Menú incluye: "Nueva actividad", "Acerca de", "Puesta a cero"
- CA-009.4: "Puesta a cero" aparece en rojo (acción destructiva)
- CA-009.5: Menú se cierra al pulsar fuera o seleccionar opción

**Relacionado con**: RF-008, RF-014, RF-015

---

### RF-010: Navegación por pestañas
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El usuario debe poder navegar entre diferentes vistas de actividades.

**Criterios de aceptación**:
- CA-010.1: Pestañas se muestran en barra horizontal con scroll
- CA-010.2: Primera pestaña siempre es "⭐ Frecuentes"
- CA-010.3: Resto de pestañas son actividades activas (no archivadas), generadas dinámicamente
- CA-010.4: Pestaña activa se destaca visualmente
- CA-010.5: Al cambiar pestaña, se actualiza grid de tareas
- CA-010.6: Pestañas tienen sticky positioning debajo del timer
- CA-010.7: En móvil, scroll horizontal funciona con swipe

**Relacionado con**: RF-001, RF-006

---

### RF-011: Modal de selección de actividad
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: Cuando un tipo de tarea puede usarse en múltiples actividades, se debe mostrar selector.

**Criterios de aceptación**:
- CA-011.1: Modal muestra icono y nombre del tipo de tarea
- CA-011.2: Modal muestra texto "¿En qué actividad?"
- CA-011.3: Se muestran botones para cada actividad permitida
- CA-011.4: Al seleccionar actividad, se abre modal de comentario
- CA-011.5: Botón "Cancelar" cierra modal sin acción
- CA-011.6: Click en overlay cierra modal sin acción

**Relacionado con**: RF-002

---

### RF-012: Persistencia de sesiones
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El sistema debe guardar todas las sesiones de trabajo mediante API REST en backend PHP+SQLite.

**Criterios de aceptación**:
- CA-012.1: Cada sesión cerrada se almacena con: ID único, tarea_id, inicio (Unix timestamp), fin, duración (segundos), notas
- CA-012.2: Sesión activa se almacena con fin = NULL
- CA-012.3: Al cerrar navegador, sesión activa NO se pierde (persiste en SQLite)
- CA-012.4: Datos persisten entre sesiones del navegador
- CA-012.5: La API incluye `server_time` en respuestas para compensar offset de reloj

**Relacionado con**: RF-002, RF-004, RF-005

---

### RF-013: Comentario de sesión
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: Al iniciar una sesión, el usuario puede añadir un comentario opcional.

**Criterios de aceptación**:
- CA-013.1: Modal de comentario aparece siempre al iniciar tarea (antes del inicio efectivo)
- CA-013.2: Campo de texto con placeholder "¿En qué concretamente?", máximo 120 caracteres
- CA-013.3: El campo es opcional; pulsar Enter o "Iniciar" sin texto arranca sin comentario
- CA-013.4: El comentario se almacena en la BD (campo `notas` de la sesión)
- CA-013.5: El comentario activo se muestra en la barra inferior izquierda (truncado si es largo)
- CA-013.6: Al cerrar sesión, el comentario desaparece de la barra inferior

**Relacionado con**: RF-002

---

### RF-014: Acerca de
**Prioridad**: Baja
**Estado**: ✅ Implementado
**Descripción**: El menú de configuración debe ofrecer información sobre el sistema y estado de conexión.

**Criterios de aceptación**:
- CA-014.1: Muestra estado de conexión con el NAS (verde/rojo + latencia)
- CA-014.2: Muestra versión de la API
- CA-014.3: Muestra tiempo acumulado total por actividad (todas las sesiones históricas)
- CA-014.4: Si no hay sesiones, muestra mensaje "Sin sesiones registradas aún"

**Relacionado con**: RF-009

---

### RF-015: Puesta a cero
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: El usuario debe poder resetear el sistema exportando el historial y archivando datos no permanentes.

**Flujo (3 fases)**:
1. **Aviso**: Descripción de lo que ocurrirá + botón "Continuar"
2. **Selección**: Lista de actividades con checkbox. Las marcadas como `permanente` aparecen pre-marcadas con 🔒. El usuario decide cuáles conservar
3. **Confirmación**: Campo de texto donde debe escribir "BORRAR" exacto + botón "Borrar todo"

**Al confirmar**:
- Se detiene la sesión activa (si existe)
- Se exporta CSV con todas las sesiones (UTF-8 con BOM para Excel)
- Se borran todas las sesiones de la BD
- Se resetea `usos_7d = 0` en todos los tipos de tarea
- Las actividades NO marcadas se archivan (`archived = 1`)
- Los tipos de tarea que quedan sin ninguna actividad activa se archivan
- La UI se regenera desde la BD

**Criterios de aceptación**:
- CA-015.1: Proceso de 3 fases con navegación Continuar/Atrás/Cancelar
- CA-015.2: Las actividades permanentes aparecen pre-marcadas con icono 🔒
- CA-015.3: Se puede marcar/desmarcar cualquier actividad libremente
- CA-015.4: Solo se ejecuta si el usuario escribe "BORRAR" exacto (mayúsculas)
- CA-015.5: Tras el reset, las pestañas de actividades archivadas desaparecen
- CA-015.6: Tras el reset, la pestaña Frecuentes queda vacía (todos `usos_7d = 0`)
- CA-015.7: El CSV se descarga automáticamente antes de borrar
- CA-015.8: Si la descarga del CSV falla (ej: móvil), el reset se ejecuta igualmente

**Relacionado con**: RF-009, RF-008

---

## 3. Requisitos No Funcionales

### RNF-001: Persistencia de datos
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: Los datos se almacenan en SQLite en disco persistente del NAS.

**Criterios de aceptación**:
- CA-NF-001.1: BD SQLite en `/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db` (disco persistente)
- CA-NF-001.2: Tablas: `actividades`, `tipos_tarea`, `tareas`, `sesiones`
- CA-NF-001.3: Datos persisten tras reinicios del NAS
- CA-NF-001.4: Arrays (`actividades_permitidas`) se serializan como JSON en columna TEXT

---

### RNF-002: Rendimiento
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: La aplicación debe responder con latencia aceptable en red local.

**Criterios de aceptación**:
- CA-NF-002.1: Iniciar sesión: < 500ms en red local (LAN)
- CA-NF-002.2: Actualización de timer: cada 1 minuto (no segundos, decisión de diseño)
- CA-NF-002.3: Cambio de pestaña: < 100ms (sin llamada a API)
- CA-NF-002.4: Carga inicial: < 3 segundos en LAN

---

### RNF-003: Usabilidad móvil
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: La interfaz debe ser óptima para uso en móvil.

**Criterios de aceptación**:
- CA-NF-003.1: Tarjetas de tareas: pulsables cómodamente con dedo (min-height: 140px)
- CA-NF-003.2: Botones modales: mínimo 44x44px
- CA-NF-003.3: Sin zoom necesario en móvil
- CA-NF-003.4: Scroll vertical suave
- CA-NF-003.5: Sin elementos que se superpongan

---

### RNF-004: Compatibilidad
**Prioridad**: Alta
**Estado**: ✅ Verificado en Chrome+Safari móvil
**Descripción**: La aplicación debe funcionar en navegadores modernos.

**Criterios de aceptación**:
- CA-NF-004.1: Chrome 90+ (desktop y móvil) ✅
- CA-NF-004.2: Safari 14+ (iOS) ✅
- CA-NF-004.3: Firefox 88+ (desktop)
- CA-NF-004.4: Edge 90+

---

### RNF-005: Portabilidad de datos
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: Los datos deben ser exportables.

**Criterios de aceptación**:
- CA-NF-005.1: Puesta a cero genera CSV con historial completo (UTF-8 BOM para Excel)
- CA-NF-005.2: CSV incluye: id, inicio, fin, duración_seg, duración_hhmm, tipo_tarea, icono, actividad, notas
- CA-NF-005.3: No hay lock-in propietario

---

### RNF-006: Disponibilidad
**Prioridad**: Alta
**Estado**: ✅ Implementado
**Descripción**: El sistema debe recuperarse automáticamente tras reinicios del NAS.

**Criterios de aceptación**:
- CA-NF-006.1: `autorestaurar.sh` en disco persistente se auto-registra en cron al arrancar
- CA-NF-006.2: Sincronización NTP via `sntp` en cron cada 5 minutos
- CA-NF-006.3: `recuperar-nas.sh` repara toda la configuración sin contraseña

---

### RNF-007: Sincronización de reloj
**Prioridad**: Media
**Estado**: ✅ Implementado
**Descripción**: El timer del cliente debe compensar diferencias de reloj entre cliente y servidor.

**Criterios de aceptación**:
- CA-NF-007.1: La API incluye `server_time` (timestamp Unix) en respuestas de sesión activa e inicio
- CA-NF-007.2: El cliente calcula `clockOffset = server_time*1000 - Date.now()`
- CA-NF-007.3: El inicio de sesión se ajusta: `inicioMs - clockOffset`
- CA-NF-007.4: El inicio se trunca al minuto: `Math.floor(ms/60000)*60000`

---

## 4. Casos de Uso Principales

### CU-001: Día típico de trabajo
**Actor**: Usuario profesional
**Objetivo**: Trackear día de trabajo con varios proyectos

**Flujo**:
1. Usuario abre aplicación por la mañana
2. Ve pestaña "Frecuentes" con sus tareas habituales (las de `usos_7d > 0`)
3. Pulsa "☕ Descanso" → Modal de comentario → Pulsa "Iniciar" (sin comentario)
4. Desayuna 20 minutos
5. Pulsa "💻 Codificar" → Modal de comentario → Escribe "API de pagos" → Pulsa "Iniciar"
6. Si Codificar tiene múltiples actividades, primero aparece el modal de actividad
7. Trabaja 2 horas. Timer muestra "02:00"
8. Al final del día, barra inferior muestra "Hoy: 8:45"

**Resultado**: Usuario tiene registro completo de su día sin fricción

---

### CU-002: Fin de proyecto
**Actor**: Usuario
**Objetivo**: Cerrar proyecto y limpiar el sistema

**Flujo**:
1. Proyecto X finalizado y cobrado
2. Usuario abre "Puesta a cero"
3. Fase 1: Lee descripción → "Continuar"
4. Fase 2: Ve lista de actividades. "Personal" y "No productivo" están pre-marcadas (🔒). Desmarca "Proyecto X" y "Proyecto Y". Mantiene marcadas Admin, Personal, No productivo
5. Fase 3: Escribe "BORRAR" → "Borrar todo"
6. CSV descargado. Pestañas Proyecto X y Proyecto Y desaparecen. Frecuentes queda vacía

---

### CU-003: Nuevo proyecto
**Actor**: Usuario
**Objetivo**: Añadir nueva actividad para nuevo cliente

**Flujo**:
1. ⚙️ → "Nueva actividad"
2. Nombre: "Proyecto Z", color, marcar "🔒 Conservar tras puesta a cero" si es proyecto largo
3. Crear → Pestaña "Proyecto Z" aparece inmediatamente
4. Botón "+" → crear tipos de tarea para ese proyecto

---

## 5. Matriz de Trazabilidad

| Requisito | Prioridad | Casos de Uso | Estado |
|-----------|-----------|--------------|--------|
| RF-001 | Alta | CU-001 | ✅ Implementado |
| RF-002 | Alta | CU-001, CU-002 | ✅ Implementado |
| RF-003 | Alta | CU-001 | ✅ Implementado |
| RF-004 | Alta | CU-001 | ✅ Implementado |
| RF-005 | Alta | CU-001 | ✅ Implementado |
| RF-006 | Media | CU-001 | ✅ Implementado |
| RF-007 | Media | CU-003 | ✅ Implementado |
| RF-008 | Media | CU-003 | ✅ Implementado |
| RF-009 | Baja | CU-002, CU-003 | ✅ Implementado |
| RF-010 | Alta | CU-001 | ✅ Implementado |
| RF-011 | Alta | CU-001 | ✅ Implementado |
| RF-012 | Alta | CU-001, CU-003 | ✅ Implementado |
| RF-013 | Media | CU-001 | ✅ Implementado |
| RF-014 | Baja | - | ✅ Implementado |
| RF-015 | Media | CU-002 | ✅ Implementado |
| RNF-001 | Alta | Todos | ✅ Implementado |
| RNF-002 | Media | CU-001 | ✅ Implementado |
| RNF-003 | Alta | Todos | ✅ Implementado |
| RNF-004 | Alta | Todos | ✅ Parcial (Chrome+Safari) |
| RNF-005 | Media | CU-002 | ✅ Implementado |
| RNF-006 | Alta | - | ✅ Implementado |
| RNF-007 | Media | CU-001 | ✅ Implementado |

---

## 6. Fuera de Ámbito (v1.x)

Los siguientes requisitos NO están incluidos en esta versión:

- **Tracking automático** de aplicaciones activas
- **Sincronización multi-dispositivo** en tiempo real
- **Modo offline** con sincronización diferida
- **Integración con calendarios** (Google Calendar, Outlook) → v2.0
- **Técnica Pomodoro** con pausas automáticas → v2.1
- **Reportes avanzados** y gráficos → v3.0
- **Estimaciones de tiempo** por tarea
- **Notificaciones** de cualquier tipo
- **Tests automatizados** (se usa verificación manual + curl)

---

## 7. Criterios de Aceptación del MVP

El MVP se considera completo cuando:

✅ **Funcionalidad Core**:
- Usuario puede crear actividades y tipos de tarea
- Usuario puede iniciar/cambiar sesiones con 1-3 taps (incluyendo comentario)
- Timer muestra sesión activa en tiempo real (resolución minutos)
- Se acumula tiempo por tarea y por día
- Puesta a cero con selección de actividades a conservar

✅ **Persistencia**:
- Datos se guardan en SQLite en disco persistente del NAS
- Datos persisten entre sesiones del navegador y tras reinicios del NAS
- Exportación a CSV en puesta a cero

✅ **Usabilidad**:
- Funciona en móvil sin zoom necesario
- Tarjetas son pulsables fácilmente
- Sin elementos superpuestos
- Timer compensa diferencia de reloj cliente/servidor

✅ **Operaciones**:
- Script de recuperación automática post-reinicio
- Deploy via SCP desde WSL sin contraseña

---

## 8. Propuestas de mejora originadas por Claude

Mejoras no solicitadas explícitamente por el usuario, identificadas e implementadas por Claude durante el desarrollo.

### PC-001: Icono renderizado en la cabecera del timer
**Versión**: 1.1.1
**Estado**: ✅ Implementado
**Descripción**: Mostrar el icono Material Symbols de la tarea activa en la cabecera del timer, en lugar del nombre de texto del icono que aparecía como texto plano al usar `textContent` en lugar de `innerHTML`.
**Motivación**: El nombre del icono (ej: `code`, `book`) resultaba innecesario e ilegible como texto; sustituirlo por el símbolo visual aporta consistencia con el resto de la UI.

---

## Apéndice A: Arquitectura Técnica

### Estructura de BD SQLite

```sql
CREATE TABLE actividades (
    id TEXT PRIMARY KEY,        -- slug del nombre
    nombre TEXT NOT NULL,
    color TEXT NOT NULL,
    created_at INTEGER,
    archived INTEGER DEFAULT 0, -- soft delete
    permanente INTEGER DEFAULT 0 -- conservar en puesta a cero
);

CREATE TABLE tipos_tarea (
    id TEXT PRIMARY KEY,
    nombre TEXT NOT NULL,
    icono TEXT,
    actividades_permitidas TEXT, -- JSON array de IDs
    usos_7d INTEGER DEFAULT 0,
    created_at INTEGER,
    archived INTEGER DEFAULT 0
);

CREATE TABLE tareas (
    id TEXT PRIMARY KEY,        -- formato: tipo_tarea_id_actividad_id
    tipo_tarea_id TEXT,
    actividad_id TEXT
);

CREATE TABLE sesiones (
    id TEXT PRIMARY KEY,
    tarea_id TEXT,
    inicio INTEGER,             -- Unix timestamp
    fin INTEGER,                -- NULL si activa
    duracion INTEGER,           -- segundos
    notas TEXT
);
```

### Endpoints API

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/actividades` | Lista actividades activas (con campo `permanente`) |
| POST | `/api/actividades` | Crear actividad |
| PUT | `/api/actividades?id=X` | Actualizar actividad (incluye `permanente`) |
| DELETE | `/api/actividades?id=X` | Archivar actividad |
| GET | `/api/tipos-tarea` | Lista tipos de tarea activos |
| POST | `/api/tipos-tarea` | Crear tipo de tarea |
| GET | `/api/sesiones?fecha=hoy` | Sesiones del día |
| GET | `/api/sesiones?action=activa` | Sesión activa (incluye `server_time`) |
| GET | `/api/sesiones?action=acumulado` | Tiempo acumulado por actividad |
| POST | `/api/sesiones?action=iniciar` | Iniciar sesión (body: `{tarea_id, notas}`) |
| POST | `/api/sesiones?action=detener` | Detener sesión activa |
| POST | `/api/sesiones?action=reset` | Puesta a cero (body: `{conservar: [...ids]}`) → devuelve CSV |
| GET | `/api/health` | Estado del servidor |

---

**Fin del Documento de Requisitos v1.1**
