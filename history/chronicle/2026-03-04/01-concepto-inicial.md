# Concepto inicial — helix-dsl-verified

**Fecha**: 4 de marzo de 2026
**Participantes**: desarrollador (experiencia 1975–2005+) + Claude Sonnet 4.6

---

## El problema que lo motiva

### Observación empírica

Un desarrollador con décadas de experiencia recuerda haber escrito programas de
cálculo matricial de estructuras en HP Basic (1978) con muy pocos problemas de
depuración. El mismo desarrollador, trabajando con Claude en una aplicación web
moderna (2026), encuentra que diagnosticar un bug de eventos touch/click requiere
tiempo desproporcionado.

¿Por qué? No es pérdida de facultades. Es que el problema ha cambiado:

| 1978 — HP Basic | 2026 — JS event-driven |
|---|---|
| Ejecución lineal y determinista | Asíncrono, multi-capa, no determinista |
| Estado explícito en variables | Estado implícito en el DOM + AppState global |
| Sin plataformas ni entornos | PC vs. móvil, caché, navegadores distintos |
| El código *es* el flujo | El flujo está disperso en decenas de funciones |

### El bug concreto

En modo edición, tocar una tarjeta en móvil abría el diálogo de sesión en lugar
del de edición. Las causas fueron:

1. **Caché del móvil** — servía código antiguo. Diagnóstico: trivial en retrospectiva,
   costoso de encontrar porque fue lo último en considerarse.
2. **Condicional `if (modoEdicion)` disperso** en cuatro sitios distintos del código.
   Uno de ellos (el listener de la pestaña Frecuentes) no tenía el guard. Diagnóstico:
   requirió leer y razonar sobre centenares de líneas.

Si el estado `modoEdicion` hubiera sido un objeto polimórfico, el cuarto sitio habría
sido un error de compilación.

---

## Las hipótesis de diseño

### 1. Doble hélice: especificación → (implementación ⊕ pruebas)

Inspirado en la doble hélice del ADN, donde cada hebra es la prueba de integridad
de la otra.

En helix-dsl-verified, un requisito expresado en el DSL generaría *dos artefactos*:

```
requisito: "al tocar una tarjeta en modo edición → abrir modal de edición"
  →  impl:  tarjetaClick() { if edit_mode → mostrarModalEditar() }
  →  test:  dado(edit_mode=true) cuando(tap_tarjeta) entonces(modal_editar.visible)
```

No son tests escritos a posteriori. Son el **reverso algebraico** de la implementación:
si la implementación dice `A → B`, el test dice `dado(A) verificar(B)`.

Ventaja para LLMs: al generar código, el LLM genera simultáneamente su verificación.
No puede "olvidar" el test porque es parte del mismo acto generativo.

### 2. Condicionales solo en factorías

Conjetura confirmada: en OO puro, todo el código condicional puede encapsularse
en métodos factoría. El resto del código es polimórfico.

```
// En lugar de esto (disperso, frágil):
function onTapCard() {
    if (AppState.modoEdicion) mostrarModalEditar();
    else iniciarTarea();
}

// El DSL generaría esto:
ModoEdicion.onTapCard()  → mostrarModalEditar()
ModoNormal.onTapCard()   → iniciarTarea()
// La factoría decide qué objeto es AppState.modo; el resto no lo sabe.
```

Consecuencia: el "olvido" de un caso no es un bug silencioso — es un método
no implementado, que el compilador/runtime detecta.

### 3. Flujos de estado como ciudadanos de primera clase

Los eventos y transiciones de estado se declaran explícitamente:

```
estado ModoEdicion:
    on tap_tarjeta → mostrarModalEditar(tarjeta.tipoId)
    on tap_pestaña_actividad → mostrarModalEditarActividad(pestaña.id)
    on tap_pestaña_frecuentes → ignorar   // explícito, no silencioso

estado ModoNormal:
    on tap_tarjeta_tipo → seleccionarTipo(tarjeta.tipoId)
    on tap_tarjeta_tarea → iniciarTarea(tarjeta.tareaId)
    on tap_pestaña → cambiarPestaña(pestaña.id)
```

El bug de hoy habría sido imposible: el caso `tap_pestaña_frecuentes` en
`ModoEdicion` habría sido `ignorar` de forma explícita, no un caso omitido.

### 4. Verificabilidad formal

El DSL debe tener semántica lo suficientemente restringida para:
- Detectar casos no manejados estáticamente
- Garantizar que toda transición de estado es intencional
- Permitir que un LLM razone sobre el programa sin ejecutarlo

Referentes: Elm (sin efectos secundarios en el modelo), TLA+ (especificación formal),
Eiffel (diseño por contrato), Idris (tipos dependientes).

---

## Lo que este DSL no resolvería

- **Bugs operacionales** (caché del navegador, diferencias de plataforma): son ruido
  externo que ningún lenguaje elimina.
- **Complejidad esencial**: si el dominio es complejo, el DSL no lo simplifica;
  lo hace *visible*.
- **Adopción**: un DSL nuevo tiene coste de aprendizaje. El ROI solo es positivo si
  el código se escribe o mantiene con ayuda de LLMs de forma habitual.

---

## Preguntas abiertas

1. ¿Qué es la unidad mínima de especificación? ¿Un requisito? ¿Una transición de estado?
2. ¿Cómo se manejan los efectos secundarios (llamadas API, DOM)?
3. ¿El DSL compila a un lenguaje destino (JS, Python...) o es interpretado?
4. ¿Cómo se integra con código convencional existente (interop)?
5. ¿Cómo se expresa la composición de estados? (p.ej. `ModoEdicion + SesiónActiva`)

---

## Próximos pasos sugeridos

- Definir la gramática mínima del DSL (ver `docs/2026-03-04-02-diseno.md`)
- Elegir un caso de uso concreto como banco de pruebas
  (el sistema de modos de mi-cronometro-psp es un candidato ideal)
- Evaluar si una sesión con Claude Opus aportaría profundidad en el diseño formal
