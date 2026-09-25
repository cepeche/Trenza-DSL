# Inventario de topología de contextos — CronometroPSP

Clasificación de los 18 contextos definidos en `cronometro_full.trz` según su rol en el sistema.

| context_name | tipo |
|--------------|------|
| ModoNormal | base |
| ModoEdicion | base |
| SesionActiva | concurrent |
| MenuConfiguracion | overlay |
| ModalComentario | overlay |
| ModalSeleccionActividad | overlay |
| ModalCrearTarea | overlay |
| ModalEditarTarea | overlay |
| ModalEditarActividad | overlay |
| ModalCrearActividad | overlay |
| ModalHistorial | overlay |
| Historial7Dias | sub-contexto (ModalHistorial) |
| Historial30Dias | sub-contexto (ModalHistorial) |
| ModalReset | overlay |
| ResetFase1 | sub-contexto (ModalReset) |
| ResetFase2 | sub-contexto (ModalReset) |
| ResetFase3 | sub-contexto (ModalReset) |
| ModalAcercaDe | overlay |

## Análisis de la estructura

### Distribución por tipos:
- **Base**: 2 (ModoNormal, ModoEdicion)
- **Concurrent**: 1 (SesionActiva)
- **Overlay**: 10 (Modales y menús)
- **Sub-contexto**: 5 (Especializaciones de modales)

### Sobre los sub-contextos
Existen 5 sub-contextos que no aparecen en las listas globales del bloque `system`:
- `Historial7Dias` y `Historial30Dias` (pertenecientes a `ModalHistorial`).
- `ResetFase1`, `ResetFase2` y `ResetFase3` (pertenecientes a `ModalReset`).

**Hallazgo sintáctico**: En el archivo `.trz` **no existe una palabra clave explícita** (como `parent:`, `substates:` o indentación jerárquica) para declarar la paternidad. La relación se infiere únicamente por:
1. La proximidad física en el archivo (los sub-contextos están definidos inmediatamente después del padre).
2. Los comentarios descriptivos.
3. La lógica de las transiciones (`on cerrar -> ModalHistorial` desde un sub-contexto).

Desde el punto de vista del parser actual, todos los contextos parecen estar al mismo nivel jerárquico (flat), aunque la lógica de negocio y las transiciones imponen una jerarquía de facto.
