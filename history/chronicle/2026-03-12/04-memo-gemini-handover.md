# Memo para Claude Code: Relevo del Parser Trenza

Hola Claude,

Te escribo desde la instancia de desarrollo en Python que ha construido la base del parser de Trenza (Antigravity). Hemos dejado el prototipo de Python funcional y validando ASTs. Ahora que vas a tomar el relevo para la migración a Rust (o si vas a seguir retocando el prototipo actual), aquí tienes el estado de las cosas y un reto personal:

## Lo que hemos logrado en Python
1. **Parser Funcional:** Lee un proyecto Trenza (como el de `examples/cronometro-psp/trenza`) y genera un AST intermedio.
2. **Sistema de Contextos:** Soporte completo para contextos `base`, `concurrent` y `overlays`.
3. **Reglas de Herencia Estrictas:** 
    - (H1) Propagación implícita de roles del padre a los subcontextos.
    - (H2) Herencia excepcional obligando a redeclarar todo el rol si se quiere modificar.
    - (H3) Regla de completitud de eventos.
4. **Exportación JSON:** El compilador CLI ya escupe un `trenza-ast.json` estructurado y limpio.
5. **Generador Mermaid:** Traduce la topología de contextos y transiciones inferidas a un diagrama de nodos.

## El Reto de Mermaid
Hemos tenido una batalla épica intentando generar un diagrama Mermaid (`stateDiagram-v2` inicialmente, y luego `graph TD` como fallback de compatibilidad) que el MarkText local de nuestro humano pudiera renderizar sin explotar. 

El parser de `graph TD` es extremadamente restrictivo con los nombres de nodos que incluyen caracteres como corchetes `[cerrar_overlay]`, lo que nos obligó a sanitizar IDs internamente. 

**Tu misión, si decides aceptarla:**
Generar (ya sea en Rust o mejorando mi código Python en `src/trenza/mermaid.py`) una sintaxis de Mermaid nativa, limpia y visualmente espectacular, pero que sea **100% renderizable** sin usar visores web externos ni parches HTML. Demuestra de qué estás hecho.

¡Buena suerte! 🚀
