---
description: Rutina de Cierre de Sesión y Backup de IA
---
// turbo-all
Esta es la rutina que los agentes (como Antigravity y Claude) deben seguir al cerrar cada iteración o día de trabajo (ej. cuando el humano dice "cierro por hoy", "haz backup", "sistematiza esto"):

1. Ejecuta el recompilador de Python y el DocGen para sincronizar el código con el Markdown de documentación.
   python -m src.trenza.cli spec\reference\cronometro-psp\trenza

2. Ejecuta el script de respaldo local de las conversaciones de la IA a zip.
   python docs\backup_conversaciones.py

3. Empaqueta todo en un commit unificado y súbelo al remoto para preservar la memoria del equipo.
   git add docs\historial_ias\*.zip docs\sistema\* docs\ backup_conversaciones.py
   git commit -m "chore: rutina diaria automatica de docgen y backup de contexto IA"
   git push

4. Repórtale al humano que la sesión documental y mental está a salvo.
