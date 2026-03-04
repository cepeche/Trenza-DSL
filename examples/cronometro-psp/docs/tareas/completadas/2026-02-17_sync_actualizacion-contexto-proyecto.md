# Sincronización de Contexto - Proyecto Mi Cronómetro PSP

**Fecha**: 17/02/2026 18:49 CET  
**Autor**: Claude Chat (Sonnet 4.5)  
**Destinatario**: Claude Code  
**Estado**: Información para sincronización

---

## Resumen Ejecutivo

El usuario (César) ha estado trabajando en una sesión con Claude Chat tras tu despliegue exitoso del MVP. Esta conversación ha establecido:

1. ✅ Confirmación de que el MVP está **desplegado y funcionando** en el NAS (192.168.1.71)
2. ✅ La funcionalidad en WiFi local es **perfecta**
3. ✅ Feedback enviado a Anthropic sobre integración Chat ↔ Code
4. 📋 Establecimiento de **convención de documentación** para mantener trazabilidad

---

## Cambios en el Proyecto

### 1. Nueva Estructura de Documentación

Se ha acordado crear una carpeta para documentos de trabajo:

```
docs/
├── tareas/                         # NUEVA carpeta
│   ├── completadas/                # Tareas finalizadas
│   └── [YYYY-MM-DD_tipo_descripcion.md]
├── README.md
├── requisitos.md
├── plan-pruebas.md
├── arquitectura-time-tracker.md
├── futuras-versiones.md
└── configuracion-red-local.md
```

### 2. Convención de Nombres

**Formato**: `YYYY-MM-DD_tipo_descripcion-corta.md`

**Tipos permitidos**:
- `RF-XXX` - Requisito funcional específico
- `bugfix` - Corrección de error
- `feature` - Nueva funcionalidad
- `tests` - Implementación de tests
- `refactor` - Refactorización
- `sync` - Sincronización de contexto (como este documento)

**Ejemplos**:
```
2026-02-17_RF-008_vincular-actividad-tarea-activa.md
2026-02-18_bugfix_timer-no-actualiza-cambio-dia.md
2026-02-20_tests_cobertura-sesiones-endpoint.md
```

### 3. Plantilla de Documento de Tarea

```markdown
# [Tipo] Título Descriptivo

**Fecha**: DD/MM/AAAA  
**Prioridad**: Alta/Media/Baja  
**Estado**: Pendiente/En Progreso/Completado  
**Requisito**: RF-XXX (si aplica)

---

## Contexto

[Explicación del problema o necesidad]

## Implementación

### Backend
- [ ] Checklist de cambios

### Frontend
- [ ] Checklist de cambios

## Criterios de Aceptación

- CA-XXX.X: [Criterio del requisito]

## Tests Necesarios

- TEST-XXX: [Descripción del test]

## Notas de Implementación

[Code: completa esto al implementar]
```

---

## Estado Actual del Proyecto

### ✅ Completado

- Backend PHP completo (actividades, tipos-tarea, sesiones)
- Frontend JavaScript funcional
- Schema SQLite con datos seed
- Scripts de backup y deploy
- Despliegue en NAS funcionando

### ⚠️ Identificado como Pendiente

**RF-008, CA-008.5**: Vincular automáticamente actividad nueva con tarea activa

**Ubicación del TODO en código**:
```javascript
// frontend/js/app.js - función guardarNuevaActividad()
// Líneas ~580-590 aproximadamente

// Si hay tarea activa, vincularla con la nueva actividad
let mensajeExtra = '';
if (AppState.sesionActiva) {
    const tareaActiva = AppState.tareas.find(t => t.id === AppState.sesionActiva.tareaId);
    if (tareaActiva) {
        const tipoTarea = AppState.tiposTarea.find(tt => tt.id === tareaActiva.tipoTareaId);
        if (tipoTarea && !tipoTarea.actividades_permitidas.includes(nuevaActividad.id)) {
            // TODO: Aquí deberíamos tener un endpoint para actualizar tipo de tarea
            // Por ahora solo mostramos el mensaje
            mensajeExtra = ' (nota: vincúlala manualmente con la tarea activa si es necesario)';
        }
    }
}
```

**Qué falta**:
- Endpoint `PUT /api/tipos-tarea/{id}` para actualizar `actividades_permitidas`
- Llamada desde frontend a este endpoint
- Crear tarea expandida automáticamente tras actualizar tipo de tarea

### 📊 Tests

**Estado**: `tests/` está vacío

**Requisito**: RNF-006 exige cobertura > 80%

**Prioridad**: Media (funcional para MVP, crítico para v1.0 estable)

---

## Acciones Sugeridas para Code

### Inmediatas

1. **Crear estructura de carpetas**:
   ```bash
   mkdir -p docs/tareas/completadas
   ```

2. **Mover este documento**:
   ```bash
   mv 2026-02-17_sync_actualizacion-contexto-proyecto.md docs/tareas/completadas/
   ```

3. **Verificar estado del despliegue**:
   - Confirmar que `cronometro.db` existe en NAS
   - Verificar logs de errores recientes
   - Probar endpoints críticos

### Próximas (según prioridad del usuario)

César tiene varias opciones y aún no ha decidido:

**Opción A - Completar RF-008**:
- Implementar endpoint PUT para tipos de tarea
- Actualizar frontend
- Crear test INT-006

**Opción B - Testing**:
- Configurar Vitest
- Implementar tests unitarios (timer, cálculos)
- Apuntar a cobertura > 80%

**Opción C - Uso en Producción**:
- Dejar que César use el sistema varios días
- Recoger feedback de UX real
- Atacar bugs o mejoras según aparezcan

**Opción D - Acceso Remoto**:
- Configurar DNS dinámico
- Port forwarding en router
- VPN o Tailscale

---

## Feedback Enviado a Anthropic

César ha enviado feedback via thumbs down solicitando:

1. Persistencia de conversaciones de Code (como Chat)
2. Cross-product search (buscar en Chat y Code)
3. Hand-off seamless entre Chat y Code
4. Unified conversation archive

---

## Notas de Contexto Adicional

### Restricciones del Usuario

- **WiFi local**: Funciona perfectamente (192.168.1.x)
- **Acceso remoto**: No urgente, puede esperar
- **Trazabilidad**: Muy importante para César (ingeniero de caminos con disciplina formal)
- **Tests**: Reconoce importancia pero no es bloqueante para empezar a usar

### Preferencias de Trabajo

- Prefiere conversaciones con Chat (más amigable, persisten)
- Valora la capacidad de ejecución directa de Code
- Quiere mantener documentación formal de decisiones
- Sistema de versionado con `.md` le parece óptimo

### Hardware Disponible

- **Servidor**: WD My Cloud EX2 Ultra (192.168.1.71)
- **Desarrollo**: Lenovo ThinkPad L460 y X230
- **Cliente principal**: Samsung Galaxy S25 Ultra (192.168.1.57)
- **Testing**: Múltiples tablets y móviles Samsung/Sony

---

## Preguntas para César (Code puede hacer si las necesita)

1. ¿Qué prioridad tiene para ti ahora mismo?
   - [ ] Completar RF-008
   - [ ] Implementar tests
   - [ ] Usar en producción y recoger feedback
   - [ ] Configurar acceso remoto

2. ¿Hay algún bug o comportamiento inesperado que hayas notado al probar?

3. ¿Te gustaría que exploremos alguna feature de v2.0 (calendario, Pomodoro)?

---

## Checklist de Sincronización para Code

- [ ] Leer este documento completo
- [ ] Verificar estado del despliegue en NAS
- [ ] Crear estructura `docs/tareas/` y `docs/tareas/completadas/`
- [ ] Mover este documento a `completadas/`
- [ ] Confirmar qué quiere hacer César a continuación
- [ ] Crear documento de tarea específico si procede

---

## Información de Sesión

**Conversación Chat**: Iniciada 17/02/2026 18:41 CET  
**Temas tratados**:
- Revisión del MVP desplegado
- Feedback a Anthropic sobre integración
- Establecimiento de convención de documentación
- Planificación de próximos pasos

**Archivos revisados por Chat**:
- `ARQUITECTURA-FINAL.md`
- `frontend/js/app.js`
- `backend/index.php`
- `backend/api/sesiones.php`
- `backend/db/schema.sql`
- `LEEME.md`

---

**Fin del documento de sincronización**

---

## Para César

Puedes compartir este documento con Code de dos formas:

1. **Directa**: Descárgalo y ábrelo en la carpeta del proyecto, luego dile a Code "lee este archivo"

2. **Crear estructura primero**: Dile a Code "crea la carpeta docs/tareas/ y docs/tareas/completadas/, luego guarda este contenido en docs/tareas/completadas/2026-02-17_sync_actualizacion-contexto-proyecto.md"

Code entenderá todo el contexto y sabrá exactamente dónde estáis.
