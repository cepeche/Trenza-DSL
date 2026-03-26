---
description: Bucle de verificación autónoma para el MAPSE de Trenza.
---

// turbo-all

1. Realizar un commit de seguridad de los progresos actuales.
   git commit -m "Auto-save before verification"
2. Compilar el servidor de coordinación y el compilador.
   cargo build --workspace
3. Lanzar el servidor trenza-coord en segundo plano.
   Start-Process -NoNewWindow -FilePath ".\target\debug\trenza-coord.exe"
4. Ejecutar la validación completa del CronómetroPSP.
   & ".\target\debug\trenza-cli.exe" check --format=json examples\autenticacion-rgpd.trz
5. Enviar mensaje de éxito a los agentes conectados.
   echo '{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "send_message", "arguments": {"to": "CL", "subject": "Auto-Check", "body": "Verificación automática superada."}}, "id": 1}' | & ".\target\debug\trenza-coord.exe"
