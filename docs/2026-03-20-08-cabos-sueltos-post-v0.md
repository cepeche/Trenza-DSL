# Memo: Los Cabos Sueltos y la Cuarta Hebra

**De**: Desarrollador Principal y Gemini 3.1 Pro (Antigravity)
**Para**: El equipo de diseño de Trenza (Opus, Sonnet)
**Fecha**: 20 de marzo de 2026
**Contexto**: Inmediatamente antes de congelar la especificación `v0.0.0` y ejecutar la reestructuración del repositorio, es vital dejar constancia formal de elementos conceptuales revolucionarios que han surgido (La Cuarta Hebra) y de las áreas funcionales explícitamente pospuestas (Cabos Sueltos).

---

## 1. La Evolución de la Trenza: La Cuarta Hebra

Hasta ahora, hemos conceptualizado Trenza como el generador de tres hebras (Triple-Thread Generation): Implementación, Tests, y Esquemáticos. 

Sin embargo, a raíz de una lúcida observación del Desarrollador Principal, el modelo debe actualizarse. **Hay una Cuarta Hebra: Los Requisitos.**

### Inversión del Paradigma Tradicional
En la Ingeniería de Software tradicional (Modelo en Cascada, Scrum), el flujo es direccional:
`Requisitos (Texto) -> Especificación Técnica -> Código -> Tests`

En Trenza / Traza, el ciclo se invierte por completo. La interacción inicial entre el humano y el LLM parte de una *intención* o *intuición* ("Quiero un cronómetro para Pomodoro que pueda sustituir tareas"). Esta intención se concreta interactivamente en el código `.trz`.
Por tanto:
`Intención + Discusión (La Historia) -> Especificación .trz -> [Implementación + Tests + Esquemáticos + REQUISITOS]`

**Los Requisitos no son el origen de la especificación; son un subproducto proyectado a partir de ella.** Al igual que el Generador de Documentos (`docgen.py`) lee el AST y genera Markdown, "Los Requisitos de Negocio" convencionales son simplemente una de las cuatro proyecciones de la fuente de la verdad. Si el negocio quiere saber cómo funciona el sistema, no lee un documento estático obsoleto; el sistema *le genera* los requisitos actualizados a partir del código.

*(Nota: Modificaré mi propuesta de ADR-017 en el memo anterior para reflejar esta Cuarta Hebra).*

---

## 2. Inventario de "Cabos Sueltos" (Post-v0.0.1)

Para congelar la versión `v0.0.0` con la conciencia tranquila, debemos mapear formalmente el territorio inexplorado. Estos son los "cabos sueltos" funcionales que conscientemente hemos dejado fuera de la primera especificación completa, pero que serán críticos para que Trenza escale a sistemas de producción complejos corporativos:

### A. Múltiples Roles de Usuario Final (RBAC / Identidad)
Actualmente, la especificación asume un "Usuario Universal". Las acciones y eventos suceden porque sí. En un sistema real, un evento (`on tap -> aprobarNomina()`) requiere saber *quién* hace el tap. Necesitamos diseñar cómo el lenguaje expresará permisos, niveles de acceso y segregación de funciones. 
*¿El rol del usuario es un Contexto? ¿Es un modificador de Guarda (`when`)?* Es el primer gran GAP de la versión 0.1.

### B. Versionado y Evolución del Estado (State/Data Migration)
El software vive en el tiempo. Si añadimos un nuevo estado a una máquina de estados existente, ¿qué ocurre con las sesiones o entidades de base de datos de los usuarios que estaban persistidas con la "versión anterior" del `system.trz`?
Trenza va a necesitar primitivas de migración. Dado que Trenza conoce rigurosamente el grafo de estados antiguo y el nuevo, podría incluso auto-generar las funciones de migración para pasar entidades de un estado abolido a su equivalente en el nuevo grafo.

### C. La Trenza Autogeneradora (Herramientas Visuales)
Si la especificación .trz genera el esquemático de Mermaid... la manipulación de nodos en una interfaz visual derivada del esquemático debería generar el .trz. La bidireccionalidad total de la Cuarta Hebra abrirá la puerta a interfaces no code.

---

Este memo debe preservarse en la crónica y consolidarse en los futuros documentos de diseño. No ignoramos la complejidad del mundo real (permisos, versiones); simplemente acotamos estratégicamente nuestra primera victoria.
