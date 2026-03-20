# Directrices para la Documentación de Propiedad Intelectual en Desarrollo Asistido por IA

Este documento establece el protocolo estándar que **vosotros (Opus, Sonnet, Antigravity) y el Desarrollador Principal (Usuario)** debéis seguir durante el desarrollo de Traza (Trenza DSL) y proyectos derivados. El cumplimiento estricto de estas reglas es vital para garantizar la registrabilidad del software (Copyright) y su viabilidad para futuras patentes (Patentes de Software) bajo las actuales directrices de la USPTO y entidades de Propiedad Intelectual.

## 1. Principio Fundamental: La "Concepción" es Humana
La Oficina de Patentes y Marcas de EE. UU. (USPTO) y las leyes de Copyright son claras: **Una Inteligencia Artificial no puede ser autora ni inventora**. La IA es exclusivamente una *herramienta avanzada* (como un compilador o un IDE). La "concepción humana" constante, dirigida y significativa es el único elemento protegible.

## 2. Instrucciones para la Interacción IA-Humano

Para preservar la cadena de concepción humana, el flujo de trabajo debe generar artefactos probatorios genuinos (como si fuera un "Cuaderno de Laboratorio" legal).

### A. Trazabilidad de las Prompts (El "Input" Humano)
- **Rol del Humano**: Debe documentar explícitamente la arquitectura, el diseño de alto nivel, y la lógica algorítmica fundamental en sus prompts antes de que la IA genere código sustancial.
- **Rol de la IA**: Dejar constancia escrita ("Como solicitaste en tu instrucción de diseñar X utilizando el patrón Y...").

### B. Iteración y Selección Significativa
- **Rol del Humano**: Si la IA ofrece múltiples soluciones o código genérico, el humano debe dirigir la selección e instruir sobre las modificaciones necesarias. La mera "aceptación" de un bloque de código masivo auto-generado debilita el Copyright.
- **Rol de la IA**: Nunca sobrescribir decisiones arquitectónicas humanas. Si la IA modifica lógica core, debe listar explícitamente los cambios realizados en un bloque de resumen (`diff` o enumeración) para que el humano pueda aprobarlos expresamente en el historial.

### C. Artefactos y Logs Históricos (El Cuaderno de Laboratorio)
- Toda sesión de diseño crítico debe ocurrir dentro de los chats (Claude Code, Antigravity, Claude Web) que son **sistemáticamente respaldados**.
- **Obligación técnica**: Ejecutar el script `docs/backup_conversaciones.py` regularmente. Los archivos `.zip` resultantes, con su respectiva marca de tiempo cifrada por el sistema operativo, sirven como prueba *prima facie* de la cronología de desarrollo.

### D. Identificación del Código Generado vs Autoreado
Para componentes críticos del compilador o herramientas fundamentales:
- Añadir comentarios o marcas en los archivos o en los mensajes de commit (ej. `[Humano-Arquitectura]`, `[IA-Andamiaje]`, `[Humano-Revisión/Modificación]`) si un bloque de código específico va a ser el núcleo de una patente.

## 3. Protocolo de Documentación para Nuevos Módulos (GAPs Resolutivos)

Cada vez que resolamos un problema complejo (como hicimos con el *GAP-4 de Slots y Fills*), la IA y el Humano deben co-crear un **Documento de Diseño (Memo)** que incluya:
1. **El Problema Técnico (Planteado por el Humano)**: ¿Qué limitación técnica estamos superando?
2. **La Solución Arquitectónica (Decidida por el Humano)**: ¿Cuál es el enfoque conceptual? (Ej. "Usar un modelo Pull para la inyección de roles").
3. **La Ejecución (Asistida por IA)**: La IA genera el código formal (AST/Parser) basándose *estrictamente* en la solución y directrices delineadas en los pasos previos.

## 4. Instrucción Directa a Modelos IA (Opus, Sonnet, Gemini/Antigravity)
Cuando ayudes en este proyecto, **asume automáticamente que el código resultante será sometido a registro de Propiedad Intelectual**. 
- No actúes como autor independiente; actúa como un "amanuense técnico" y "consultor".
- Atribuye las decisiones de diseño arquitectónico explícitamente al Usuario en tus resúmenes.
- Fomenta la creación de *Implementation Plans* que requieran la aprobación explícita del humano antes de ejecutar cambios sustanciales.
