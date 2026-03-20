# Ciclo de vida y versionado de sistemas Trenza

**Fecha**: 13 de marzo de 2026
**Origen**: Desarrollador Principal
**Estado**: objetivo estratégico registrado — diseño pendiente

## Contexto

La gestión del ciclo de vida de los sistemas especificados en Trenza es un
objetivo fundamental del proyecto. La preocupación viene de décadas de
experiencia del desarrollador principal con el versionado de sistemas de gran
tamaño, y se agudizó particularmente durante su participación en el
**XBRL Standards Board**, donde el versionado de taxonomías del lenguaje
presentaba problemas análogos.

## El problema

Los sistemas reales no nacen completos ni estáticos. Pasan por fases con
necesidades radicalmente distintas:

1. **Desarrollo** (concepción exploratoria): la especificación cambia
   constantemente, hay GAPs abiertos, decisiones de diseño pendientes,
   estructuras provisionales. El verificador debe ser tolerante y útil,
   no bloqueante.

2. **Preproducción**: la especificación está congelada funcionalmente pero
   sometida a revisión intensiva. El verificador debe ser estricto. Los
   cambios requieren trazabilidad explícita.

3. **Producción**: la especificación es un contrato. Los cambios son
   versionados formalmente con semántica clara de compatibilidad
   (breaking vs non-breaking).

## Requisitos de diseño (preliminares)

- Trenza debe soportar **explícitamente** estos tres estados como parte
  del lenguaje o su metadatos, no como convención externa.
- Las reglas de verificación deben adaptarse al estado del ciclo de vida:
  en desarrollo se permiten especificaciones incompletas; en producción, no.
- El versionado debe permitir razonamiento sobre **compatibilidad entre
  versiones**: ¿una nueva versión del sistema rompe contratos con versiones
  anteriores?
- La experiencia con XBRL sugiere que el versionado de taxonomías/esquemas
  tiene trampas sutiles que no se resuelven con semver ingenuo.

## Decisiones aplazadas

- Mecanismo concreto de declaración de versión (¿en manifest? ¿en system.trz?)
- Semántica de compatibilidad (¿structural? ¿behavioral?)
- Relación con el empaquetado `.tzp`
- Granularidad del versionado (¿sistema completo? ¿por contexto?)

## Prioridad

**Alta**, pero el diseño debe esperar a que la gramática y las reglas de
verificación estén estabilizadas. Registramos ahora para que ninguna decisión
de diseño cierre puertas prematuramente.

---

> *Nota*: Este objetivo se registra explícitamente como parte de los principios
> fundacionales del proyecto, al mismo nivel que los cuatro principios de diseño
> del CLAUDE.md.
