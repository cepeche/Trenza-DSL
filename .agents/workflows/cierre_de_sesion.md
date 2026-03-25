---
description: Rutina de Cierre de Sesión y Backup de IA
---
// turbo-all
Esta es la rutina que los agentes (como Antigravity y Claude) deben seguir al cerrar cada iteración o día de trabajo (ej. cuando el humano dice "cierro por hoy", "haz backup", "sistematiza esto"):

0. **Sincronización de Contexto**: Al INICIO de cada sesión, lee la última entrada en `history/chronicle/` para sincronizar con otros agentes.

1. **DocGen**: Ejecuta el recompilador de Python y el DocGen para sincronizar el código con el Markdown de documentación.
   `python -m src.trenza.cli spec\reference\cronometro-psp\trenza`

2. **Crónica de Sesión**: Escribe una nueva entrada en `history/chronicle/YYYY-MM-DD/NN_<nombre>.md` detallando las decisiones, razonamientos y estado de los artefactos. Este es el **contrato de coordinación fundamental**.

3. **Backup de Conversaciones**: Ejecuta el script de respaldo local de las conversaciones de la IA a zip.
   `python docs\backup_conversaciones.py`

4. **Commit y Push**: Empaqueta todo en un commit unificado y súbelo al remoto.
   `git add docs\historial_ias\*.zip docs\sistema\* docs\ history/chronicle\* backup_conversaciones.py`
   `git commit -m "chore: rutina diaria automatica de docgen, cronica y backup de contexto IA"`
   `git push`

5. **Reporte**: Notifica al humano que la sesión documental y mental está a salvo.
