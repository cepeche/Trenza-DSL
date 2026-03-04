# Mi Cronómetro PSP - Roadmap

**Última actualización**: 25 febrero 2026

---

## ✅ Completado

| Versión | Qué se hizo |
|---------|------------|
| v1.0 | MVP funcional: cronómetro, actividades, sesiones, puesta a cero |
| v1.1 | Modo edición, grid 4 columnas, pestañas dinámicas, inicio retroactivo, modo sustituir, aviso de recorte, bug usos_7d |
| v1.2 | Acceso remoto HTTPS + DDNS (`cronometro.hash-pointer.com`), autenticación mTLS con CA privada + certificado de cliente, suite de tests automatizados (PHPUnit + Node + smoke bash), fixes backend PHP 8.3 |

---

## 🔜 Próxima iteración — v1.3

### Funcionalidad
- [ ] **Vista historial**: resumen diario/semanal con distribución de tiempo por actividad
  - Gráfico de barras o tabla: tiempo por tipo de tarea en los últimos N días
  - Acceso desde menú o pestaña dedicada
- [ ] **Editar tipos de tarea existentes**: cambiar nombre e icono
- [ ] **Editar actividades existentes**: cambiar nombre y color
- [ ] **Editar flag `permanente`** en actividades ya creadas (ahora solo al crearlas)

### Infraestructura
- [ ] **Backup automático al NAS secundario** (192.168.1.68 o 192.168.1.75):
  `rsync` de `cronometro.db` programado, integrado en `autorestaurar.sh`

---

## 🗓️ Backlog — v2.x

### Usabilidad
- [ ] Ordenar manualmente el grid de tareas (drag & drop o flechas)
- [ ] Añadir nuevo dispositivo con certificado de cliente (actualmente proceso manual; valorar un script asistido)

### Integración con calendario
- Consultar el siguiente evento al iniciar una tarea
- Notificación visual: "⏰ Próximo evento en 25 min: Reunión"
- Google Calendar (prioritario) y CalDAV

### Técnica Pomodoro
- Timer configurable (25 min trabajo + 5 min pausa)
- Pausas sin cerrar la sesión activa
- Estadísticas de Pomodoros completados

### Infraestructura
- [ ] Backup automático a NAS secundario (192.168.1.68/75) — alta prioridad
- [ ] Recordatorios inteligentes ("llevas 2 días sin Proyecto X")
- [ ] Widget Android / PWA instalable

---

## 📋 Principios de diseño

1. **Fricción mínima**: cualquier feature nueva debe reducir, no aumentar, el esfuerzo de tracking
2. **Opcional por defecto**: features avanzadas deben ser opt-in
3. **Respeto del flujo**: nunca interrumpir bruscamente el trabajo
4. **Datos bajo control**: siempre exportables, siempre transparentes
5. **Privacidad primero**: sin dependencias de servicios externos críticos
6. **Sin Cloudflare**: bloqueado judicialmente en España en varias ocasiones
