# The Power of Ten: Rules for Developing Safety-Critical Code

**Autor**: Gerard J. Holzmann (NASA / Jet Propulsion Laboratory)
**Origen**: IEEE Computer Society, 2006.
**Motivo de inclusión**: Referencia fundacional para el diseño de Trenza-DSL (sesión 18 marzo 2026). La Regla 3 inspira la asunción de falibilidad por defecto en las acciones asíncronas de Trenza y el rechazo de la magia implícita.

---

Estas son las diez reglas originales establecidas por el JPL para el desarrollo de software crítico para misiones espaciales (escritas originalmente orientadas a C, pero cuyos principios arquitectónicos son adaptables a cualquier sistema de alta fiabilidad):

1. **Restringir todo el código a flujos de control muy simples**: no usar declaraciones `goto`, conversiones de salto directas o indirectas (`setjmp` o `longjmp`), ni llamadas a bloqueos explícitos o explícitos de subprocesos. No usar recursividad directa o indirecta.
2. **Límites fijos en los bucles**: Todos los bucles deben tener un límite superior fijo. Debe poder demostrarse de forma trivial que el límite superior no puede superarse.
3. **No usar asignación dinámica de memoria después de la inicialización**: Está prohibida toda asignación/desasignación de memoria tras la configuración inicial (ej. `malloc()`). *[Nota Trenza: Inspira la exigencia de modelar estáticamente todas las ramificaciones de error]*.
4. **Funciones cortas**: Ninguna función debe ser más larga de lo que se puede imprimir en una sola hoja de papel (unas 60 líneas de código).
5. **Densidad de aserciones**: La densidad de aserciones del código debe tener un promedio mínimo de dos aserciones por función. Las aserciones deben usarse para comprobar anomalías que no deberían ocurrir nunca en la vida real.
6. **Alcance mínimo**: Los objetos de datos deben declararse en el nivel de alcance (scope) más pequeño posible.
7. **Comprobar valores de retorno y parámetros**: La función que llama debe comprobar el valor de retorno de las funciones no-nulas, y la función llamada debe comprobar la validez de todos los parámetros que se le pasan.
8. **Uso limitado del preprocesador**: El uso del preprocesador debe limitarse a la inclusión de archivos de cabecera y el uso de simples macros. No se permiten macros con parámetros ni compilación condicional.
9. **Restringir el uso de punteros**: El uso de punteros debe restringirse a un único nivel de desreferencia. No se permiten los punteros a funciones.
10. **Compilar con todas las advertencias (Warnings como Errors)**: Todo el código debe compilarse, desde el primer día de desarrollo, con todas las advertencias del compilador habilitadas en la configuración más pedante. Todas las advertencias deben tratarse como errores.
