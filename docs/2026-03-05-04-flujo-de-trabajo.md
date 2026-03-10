# Flujo de trabajo y el papel de los requisitos

**Fecha**: 4 de marzo de 2026

---

## El problema del vibe coding sin trazabilidad

El "vibe coding" — generar código directamente desde lenguaje natural sin
estructura intermedia — no es un problema de calidad del LLM. Es un problema
de proceso: el LLM genera código sin que nadie haya definido la topología
del sistema. Es como construir un circuito soldando componentes al azar y
esperando que funcione.

Los métodos ágiles sin TDD tienen el mismo problema: la velocidad inicial
de desarrollo se paga con deuda técnica acumulada y bugs estructurales como
el que motivó este proyecto — cuatro listeners, uno sin guard, imposible de
detectar sin leer centenares de líneas.

**Helix es la respuesta a ambos problemas simultáneamente.**

---

## La inversión del flujo

### Flujo tradicional

```
requisitos (humano)
    → código (humano / LLM)
    → tests (humano / LLM, a posteriori)
    → documentación (nadie, o demasiado tarde)
```

Los requisitos son el artefacto primario. El código los implementa.
Los tests los verifican. La documentación los describe (cuando existe).
Cada artefacto es creado por un actor diferente en un momento diferente,
con riesgo de inconsistencia en cada salto.

### Flujo Helix

```
conversación (humano + LLM)
    → .helix (LLM genera la topología)
    → código + tests + esquemático Mermaid (compilador determinista)
    → requisitos (resumen para humanos, generado del .helix)
```

Los requisitos son un **artefacto derivado**, no primario. Son el resumen
legible por humanos de una especificación formal que ya existe en el `.helix`.
La consistencia entre código, tests y documentación está garantizada por
construcción — todos vienen del mismo artefacto fuente.

---

## Por qué esto es compatible con metodologías ágiles

La objeción obvia es: "los requisitos emergen de la interacción con el
prototipo, no se pueden especificar de antemano". Es cierta, y Helix
no la contradice.

La clave es que el `.helix` puede generarse **durante** la fase exploratoria,
no antes. El flujo conversacional con el LLM que antes producía código
directamente ahora produce `.helix` primero. La diferencia no es que haya
más trabajo previo — es que el trabajo de estructurar el problema, que el
LLM hacía implícitamente al generar código, ahora se hace explícito y se
preserva como artefacto.

Si el LLM va a generar código, es porque ya ha definido internamente su
estructura y funcionalidad. Hacer ingeniería inversa de ese código para
documentarlo en Helix sería pasar dos veces por lo mismo. Helix captura
esa estructura en el momento en que se define, no después.

**La fase exploratoria con Helix no es más lenta — es la misma velocidad
con trazabilidad incluida.**

---

## Los requisitos como resumen para humanos

En un proyecto Helix maduro, el documento de requisitos no se escribe —
se genera. El `.helix` contiene toda la información necesaria para
producir automáticamente:

- El esquemático Mermaid (para que el humano valide la topología
  visualmente)
- Un documento de requisitos en lenguaje natural (para comunicar con
  stakeholders no técnicos)
- La lista de casos de uso (directamente de los Contexts)
- La matriz de cobertura tests/requisitos (por construcción, es siempre
  100%)

Ninguno de estos artefactos necesita ser mantenido manualmente. Son
proyecciones del `.helix`.

---

## Conexión con TDD

TDD llegó a una conclusión similar por otro camino: si no puedes escribir
el test antes que el código, es que no entiendes lo que estás construyendo.
El test forzado a priori es un mecanismo para obligar a pensar en la
topología antes de implementarla.

Helix dice lo mismo un nivel más arriba: si no puedes escribir el `.helix`
antes que el código, es que no entiendes la topología del sistema. Y si
el LLM genera el `.helix` como paso previo explícito, la "comprensión"
que antes quedaba implícita en el proceso generativo queda ahora capturada
y verificable.

---

## Implicación para el diseño del CLI

El comando principal del CLI no debería ser `helix build` (compilar) sino
algo que refleje el flujo conversacional:

```
helix verify    # ¿Es este .helix válido? ¿Hay casos no manejados?
helix build     # Generar código + tests + esquemático
helix summarize # Generar resumen en lenguaje natural para humanos
```

El LLM usa `helix verify` en su bucle interno antes de entregar al humano.
El humano usa `helix summarize` para comunicar con stakeholders.
`helix build` es el paso final cuando la topología está acordada.
