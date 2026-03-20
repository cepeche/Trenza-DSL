# Trenza CLI

La interfaz de línea de comandos de Trenza permite inicializar, verificar y compilar proyectos estáticamente sin salir del terminal.

## Comandos Principales de la Herramienta
- `trenza init <project>`: Crea la estructura básica de carpetas y andamiaje `.trz`.
- `trenza verify`: Compila en seco y verifica las 6 reglas formales. Emite los errores legibles si falta por ejemplo un handler para un estado.
- `trenza generate`: Genera las cuatro hebras (implementación, tests, esquemático y requisitos). Genera Rust por defecto.
- `trenza package`: Compila un subsistema entero creando el paquete firmado `.tzp`.
