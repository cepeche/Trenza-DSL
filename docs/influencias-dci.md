# Influencias: DCI y Reenskaug

**Fecha**: 4 de marzo de 2026

---

## Contexto

Durante el análisis crítico del diseño emergió una conexión no documentada
pero fundamental: las hipótesis de helix-dsl-verified son, en gran medida,
una redescubierta de ideas que Trygve Reenskaug lleva décadas desarrollando.
Este documento registra esa conexión para que sea explícita en el diseño.

---

## DCI: Data, Context, Interaction

DCI es un paradigma propuesto por Reenskaug (inventor del MVC original) y
James Coplien, elaborado en OORam y sus trabajos posteriores sobre
"objetos desnudos" (*naked objects*).

La separación fundamental:

- **Data** — lo que el sistema *es*: objetos del dominio con identidad y
  estado, sin comportamiento dependiente del contexto en que se usan.
  Una tarjeta es una tarjeta; no sabe si está en modo edición o no.

- **Context** — el caso de uso activo: `ModoEdicion`, `ModoNormal`.
  El Context instancia los objetos del dominio y les asigna **roles**.
  Es el único lugar del sistema que sabe en qué escenario se encuentra.

- **Interaction** — lo que el sistema *hace* en ese Context: los métodos
  de rol que implementan el comportamiento del caso de uso.

---

## La conexión con helix

Las hipótesis de diseño de helix se alinean directamente con DCI:

### Hipótesis 2: condicionales solo en factorías

La factory que decide qué Context instanciar es exactamente el único
condicional necesario. Una vez instanciado el Context, el resto del código
es polimórfico. Esto no es una conjetura de diseño: es la consecuencia
directa de la separación DCI.

### Hipótesis 3: flujos de estado explícitos

Un estado del DSL de helix *es* un Context DCI. La sintaxis propuesta:

```
estado ModoEdicion:
    on tap_tarjeta → mostrarModalEditar(tarjeta.tipoId)
    on tap_pestaña_frecuentes → ignorar
```

puede reescribirse en terminología DCI sin pérdida de semántica:

```
context ModoEdicion:
    role tarjeta:
        on tap → mostrarModalEditar(self.tipoId)
    role pestaña_frecuentes:
        on tap → ignorar
```

La tarjeta no "sabe" que está en modo edición. El Context le asigna el rol
`tarjeta` con ese comportamiento. Fuera de ese Context, la tarjeta es un
objeto Data sin comportamiento contextual.

### Explosión de estados: solución DCI

El problema de composición `ModoEdicion + SesionActiva` se disuelve en DCI:
no es un estado combinado de un objeto — son dos Contexts que coexisten.
Cada uno gestiona sus propios roles. La composición no es multiplicativa
sino aditiva.

---

## Traceabilidad de requisitos

Una propiedad clave de OORam que helix hereda: cada Context corresponde
directamente a un caso de uso del dominio. El código tiene la misma
granularidad que los requisitos. Esto facilita:

- Que un LLM razone sobre el código en términos de casos de uso, no de
  implementación.
- Que un cambio de requisito tenga una localización clara en el código.
- Que la especificación helix sea legible por no-programadores (diseñadores,
  analistas) que piensan en casos de uso.

---

## Sobre la originalidad

Helix no inventa estas ideas. Las reconecta con el problema concreto del
desarrollo frontend moderno con LLMs, donde:

1. Los frameworks actuales (React, Vue) no imponen separación DCI.
2. Los LLMs generan código con condicionales dispersos porque así está
   la mayor parte del código en su corpus de entrenamiento.
3. El coste de diagnosticar un bug como el que motivó este proyecto
   (cuatro listeners, uno sin guard) es desproporcionado precisamente
   porque la separación DCI no está presente.

El valor de helix no es conceptual sino operativo: dar a esas ideas una
forma concreta (DSL + compilador + co-generación de tests) que funcione
en el flujo de trabajo humano/LLM de 2026.

---

## Referencias

- Reenskaug, T. y Coplien, J.: *The DCI Architecture: A New Vision of
  Object-Oriented Programming* (2009)
- Reenskaug, T.: OORam (Object-Oriented Role Analysis and Modeling)
- Hoare, C.A.R.: *The Emperor's Old Clothes* — Turing Award Lecture (1980).
  Fuente de la cita: "Hay dos tipos de software: el que es tan sencillo
  que obviamente no contiene errores, y el que es tan complicado que no
  contiene errores obvios."
