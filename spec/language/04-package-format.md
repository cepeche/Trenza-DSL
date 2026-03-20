# El Paquete Trenza (.tzp)

Trenza compila especificaciones en paquetes autocontenidos `.tzp` (Trenza Package), originalmente conocidos por la extensión legacy `.helixpkg`.

## Estructura del Paquete
Un paquete `.tzp` es un contenedor zip protegido criptográficamente que incluye:
- El código fuente `.trz` compilado.
- Los test harnesses autogenerados.
- Las implementaciones WASM embebidas.
- Un `manifest.json` que firma y vincula matemáticamente todos los componentes desde el AST.

## Ejecución Autónoma
El compilador impone el principio arquitectónico de *soluciones autocontenidas* (rUv): el output distribuible es completamente autónomo. Cualquier sistema que consuma un `.tzp` hereda instantáneamente las garantías estáticas formadas en origen.
