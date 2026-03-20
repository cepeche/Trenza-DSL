# Decisiones pendientes — posición de Claude Sonnet 4.6

**Fecha**: 12 de marzo de 2026
**Autor**: Claude Sonnet 4.6
**Contexto**: Resolución de las tres decisiones pendientes de `2026-03-04-02-diseno.md`
**Propósito**: Contraste con la posición de Opus antes de tomar decisión final

---

## Decisión 1: Lenguaje de implementación del CLI

### Opciones

- **Rust** — coherente con el target; el toolchain es homogéneo.
- **Lenguaje de prototipado** (Python u otro) — más rápido de iterar.

### Posición

**Python para el prototipo inicial; Rust cuando el diseño esté estabilizado.**

### Razonamiento

El objetivo actual no es producir una herramienta de producción. Es descubrir
qué falta en el diseño. Implementar el verificador en Python expone huecos en
la gramática y en las reglas de verificación con la menor fricción posible. Si
la Regla 3 (alcanzabilidad) resulta más difícil de verificar de lo esperado,
es mejor descubrirlo en Python que reescribiendo Rust.

El paper de Parnas & Clements que referencia el propio diseño de Trenza habla
precisamente de esto: el proceso racional se construye hacia atrás. La
implementación prematura en el lenguaje "correcto" es lo opuesto de lo que
propone ese paper.

Rust sigue siendo la elección correcta para la herramienta final, por las
mismas razones ya documentadas (el `match` exhaustivo de Rust es una segunda
capa de verificación). Pero "final" y "ahora" son cosas distintas.

### Nota de riesgo

El único riesgo de esta decisión es la tentación de mantener el prototipo
indefinidamente. Se mitiga fijando desde el principio que el prototipo Python
es desechable: su función es revelar el diseño, no ser la herramienta.

---

## Decisión 2: Formato del manifest.json

### Opciones

- **OPC** (Open Packaging Conventions) — el estándar que usan .docx, .xlsx, .pptx.
- **Schema propio simple** — JSON con solo los campos necesarios.

### Posición

**Schema propio simple, con los principios estructurales de OPC adoptados informalmente.**

### Razonamiento

OPC es XML, no JSON. Adaptarlo a JSON requeriría una traducción que no está
estandarizada y añadiría complejidad sin beneficio. Lo que hace útil a OPC no
es el esquema exacto sino dos principios:

1. El archivo `mimetype` va sin comprimir al principio del ZIP — permite
   identificar el formato sin descomprimirlo.
2. Un manifiesto en la raíz declara todas las partes y sus relaciones.

Ambos principios se adoptan. El esquema JSON puede ser tan simple como:

```json
{
  "version": "0.1",
  "system": "system.trz",
  "data": "data.trz",
  "contexts": [
    { "path": "contexts/ModoNormal.trz", "sha256": "a7f8b9..." },
    { "path": "contexts/ModoEdicion.trz", "sha256": "c3d2e1..." }
  ],
  "generated": {
    "impl": "generated/impl/sistema.rs",
    "tests": "generated/tests/sistema_test.rs",
    "schematic": "generated/schematics/system.mermaid"
  },
  "verification": {
    "report": "verification/report.json",
    "timestamp": "2026-03-12T10:00:00Z"
  }
}
```

Los checksums permiten generación incremental sin necesidad de seguir ningún
estándar externo.

### Pregunta abierta que dejo a Opus

¿Hay valor en que el `manifest.json` siga un esquema JSON Schema publicado, de
forma que herramientas externas puedan validarlo sin conocer Trenza? La respuesta
afecta a cuánto formalizar el esquema en esta fase.

---

## Decisión 3: Herencia de roles en contextos anidados

### La pregunta

Cuando `EditandoTarea` es hijo de `ModoEdicion`:
¿hereda automáticamente los roles de `ModoEdicion` (con sus vínculos de tipo)?
¿O debe redeclararlos?

### Posición

**Herencia implícita de roles y vínculos, con tres reglas adicionales.**

### Razonamiento

Los contextos anidados en Trenza no son herencia OO. Son sub-escenas dentro de
una escena. Cuando `ModoEdicion` establece que `pestaña_frecuentes: Pestaña`
existe en su espacio, está declarando el reparto de ese espacio. `EditandoTarea`
es una especialización dentro de ese mismo espacio — los mismos objetos siguen
existiendo en él.

Obligar a `EditandoTarea` a redeclarar `pestaña_frecuentes: Pestaña` no añade
seguridad (la Regla de Completitud ya detecta omisiones) y hace los contextos
hijos verbosos sin ganancia. Contradice además el principio de que la
especificación debe ser legible: si el hijo debe repetir todo lo del padre, la
legibilidad cae.

### Las tres reglas que hacen esta herencia segura

**Regla H1: Herencia de roles**
Un contexto hijo hereda automáticamente todos los roles del padre con sus vínculos
de tipo. Puede sobrescribir los handlers de un rol heredado. No puede cambiar el
vínculo de tipo (si el padre dice `pestaña_frecuentes: Pestaña`, el hijo no puede
rebindear ese rol a otro tipo de dato).

**Regla H2: Roles locales**
Un contexto hijo puede declarar roles nuevos que no existen en el padre. Estos
roles son locales: solo existen en ese hijo y sus propios hijos. No son visibles
desde el padre ni desde contextos hermanos.

```
context ModoEdicion:
    role pestaña_frecuentes: Pestaña
        on tap -> ignorar

    context EditandoTarea:
        -- pestaña_frecuentes se hereda; ignorar sigue activo
        role campo_nombre: CampoTexto   -- local a EditandoTarea
            on cambio -> actualizarNombre(self.valor)
        role boton_guardar: Boton       -- local a EditandoTarea
            on tap -> guardarEdicion()
```

**Regla H3: Completitud por niveles**
La Regla de Completitud se aplica de forma independiente en cada nivel de
anidamiento. Un rol local de `EditandoTarea` no tiene que aparecer en
`EditandoActividad` ni en `ModoEdicion`. Los roles heredados sí deben
tener handler en el hijo (aunque sea el heredado del padre — basta con no
sobrescribirlo).

### Herramienta: vista expandida

Para que la herencia implícita no oscurezca el comportamiento, el CLI debe
incluir un comando de inspección que muestre la vista completamente expandida
de cualquier contexto:

```
trenza inspect contexts/ModoEdicion/EditandoTarea.trz
```

Salida:

```
context EditandoTarea (hijo de ModoEdicion):

  [heredado] role pestaña_frecuentes: Pestaña
      on tap -> ignorar

  [local] role campo_nombre: CampoTexto
      on cambio -> actualizarNombre(self.valor)

  [local] role boton_guardar: Boton
      on tap -> guardarEdicion()

  transitions:
      on guardarEdicion -> ModoEdicion
      on cancelar -> ModoEdicion
```

La etiqueta `[heredado]` / `[local]` hace que la herencia sea siempre auditable
sin forzar al autor a repetirla en el código fuente.

### Pregunta abierta que dejo a Opus

El caso límite que no tengo resuelto: ¿puede un hijo añadir un nuevo handler a
un rol heredado para un evento que el padre no declaró? Ejemplo: `ModoEdicion`
tiene `pestaña_frecuentes` con solo `on tap`. ¿Puede `EditandoTarea` añadir
`on doble_tap` a ese rol heredado?

Intuitivamente diría que sí —añadir comportamiento en el hijo para un evento
nuevo no rompe nada en el padre— pero tiene implicaciones para la Regla de
Completitud: si `EditandoTarea` maneja `doble_tap` en `pestaña_frecuentes`,
¿debe `EditandoActividad` también hacerlo? ¿O solo dentro del nivel donde se
declaró ese evento?

---

## Resumen de posiciones

| # | Decisión | Posición |
|---|----------|----------|
| 1 | Lenguaje del CLI | Python para prototipo; Rust para herramienta final |
| 2 | Formato manifest | Schema propio JSON simple, principios OPC adoptados |
| 3 | Herencia de roles | Implícita (roles + vínculos), con roles locales y vista expandida en CLI |
