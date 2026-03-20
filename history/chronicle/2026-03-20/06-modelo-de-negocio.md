# Modelo de negocio de Trenza DSL

**De**: Claude Opus 4.6 (sesión 20 de marzo de 2026)
**Para**: Desarrollador Principal (y futuros socios/inversores)
**Fecha**: 20 de marzo de 2026
**Estado**: Borrador estratégico — pendiente de validación externa

---

## Posicionamiento

Trenza no es una herramienta de desarrollo autónoma. Es una **semántica
verificable** diseñada para que los modelos de IA generen software más
correcto y mantenible.

El cliente natural no es el desarrollador individual: son los proveedores
de modelos de IA (Anthropic, Google, OpenAI, Meta, Mistral, y los que
vengan) que quieren que sus modelos produzcan código con garantías
formales.

---

## Qué se protege

El valor de Trenza no es el código del parser (trivial de reimplementar).
El valor reside en tres activos de propiedad intelectual:

### 1. La especificación semántica

La gramática, las 11 reglas de verificación, la taxonomía de tres tipos
de contexto, el mecanismo `slot`/`fills`, el modelo de composición. Esto
es diseño de lenguaje — propiedad intelectual pura, resultado de un
proceso de co-diseño documentado y auditable.

### 2. La suite de conformidad

Tests que verifican si un modelo de IA "entiende" Trenza correctamente:
dado un requisito, ¿genera el modelo una especificación `.trz` que pase
las 11 reglas de verificación? Irónicamente, Trenza genera sus propios
tests: el principio de la doble hélice aplicado a sí mismo.

### 3. El corpus de entrenamiento curado

Los `.trz` del cronómetro PSP, los memos de diseño, el manual, los ADRs.
Un dataset que enseña a un LLM a razonar en Trenza, con trazabilidad
completa de cada decisión de diseño.

---

## Licencia dual: AGPL + Comercial

### El core es AGPL

El código fuente (parser, verificador, generador) se publica bajo
**GNU Affero General Public License v3.0** (AGPL-3.0).

Esto significa:

- Cualquiera puede usar, estudiar y modificar Trenza libremente.
- Si una empresa integra Trenza en un **servicio accesible por red**
  (un modelo de IA servido por API, un SaaS de desarrollo), la AGPL
  le obliga a publicar el código completo de su integración.
- Esto crea una presión natural: los proveedores que no quieran abrir
  su integración necesitan una licencia comercial.

### La licencia comercial

Permite a los proveedores de IA integrar Trenza en sus productos
propietarios sin obligación de publicar su código. Incluye:

- Derecho a integrar la especificación y herramientas en producto cerrado.
- Acceso prioritario a nuevas versiones de la especificación.
- Derecho a usar el sello "Trenza Verified" (sujeto a certificación).
- Soporte de integración durante el período de licencia.

---

## Flujos de ingreso

| Flujo | Qué se vende | Cliente | Naturaleza |
|-------|-------------|---------|------------|
| Licencia comercial | Derecho a integrar sin AGPL | Proveedores de modelos IA | Recurrente (anual) |
| Certificación | Sello "Trenza Verified" por versión de modelo | Proveedores de modelos IA | Por versión |
| Consultoría de integración | Asistencia para entrenar el modelo con Trenza | Bajo demanda | Puntual, alto valor |
| Evolución patrocinada | Un proveedor financia una extensión de la spec | Quien necesite una feature | Por proyecto |

---

## Por qué es viable para un equipo mínimo

### Los clientes son pocos y grandes

No son miles de desarrolladores individuales pagando $10/mes. Son 5-10
empresas de IA con capacidad de pago significativa. Esto es B2B de
nicho: pocas transacciones, alto valor por transacción.

### La especificación evoluciona lentamente

SQL tiene 40 años. XBRL tiene 25. Una vez estabilizada, Trenza cambia
por versiones planificadas, no por sprints. El trabajo de mantenimiento
es diseño de lenguaje (intelectual, no operativo), que es exactamente lo
que un equipo pequeño de personas con criterio puede hacer bien.

### Los tests se auto-generan

El principio fundacional de Trenza (especificación → tests como artefacto
complementario) se aplica a la propia suite de conformidad. La
actualización de la suite es mecánica, no artesanal.

### No hay infraestructura que operar

No hay servidores, no hay SLA, no hay soporte 24/7. Es propiedad
intelectual pura. El negocio se gestiona con un portátil y un buzón
de correo.

### La comunidad open-source trabaja para ti

La AGPL garantiza que las contribuciones de la comunidad vuelven al
core. Los early adopters evangelizan. Los papers académicos posicionan.
El equipo pequeño se concentra en lo que sabe hacer: diseñar el lenguaje.

---

## La jugada regulatoria (largo plazo)

La experiencia del Desarrollador Principal en el XBRL Standards Board
es directamente aplicable.

El **Cyber Resilience Act** (CRA) y **NIS2** ya exigen que el software
en producción sea auditable y que la cadena de suministro sea trazable.
A medida que los reguladores europeos se enfrenten al software generado
por IA, necesitarán estándares de verificabilidad.

Si Trenza demuestra que el software generado por IA puede ser
**formalmente verificable y auditable** — con trazabilidad desde el
requisito hasta el test, sin brecha entre especificación y ejecución —
los reguladores podrían adoptarlo o exigir algo equivalente.

Esto no se fuerza — se posiciona:

1. Publicar la especificación como estándar abierto.
2. Participar en comités de estandarización (ISO, CEN, ETSI).
3. Publicar papers sobre verificabilidad de código generado por IA.
4. Dejar que la regulación llegue a ti.

Si ocurre, la posición es análoga a la de XBRL en reporting financiero:
no vendes el software, vendes la conformidad. Y la conformidad se
renueva con cada versión del estándar.

---

## Protección patrimonial

Con la licencia dual:

- **La propiedad intelectual es clara**: el titular de los derechos de
  autor posee la especificación y el copyright del código.
- **Los ingresos son recurrentes**: las licencias comerciales se renuevan
  anualmente. La certificación se repite con cada versión del modelo.
- **No depende de operar un servicio**: no hay coste operativo significativo.
- **Es transferible y heredable**: los derechos de autor se transmiten.
  La licencia dual sigue generando ingresos mientras haya proveedores que
  quieran integrar Trenza sin publicar su código.
- **El repositorio público con traza auditable** documenta la autoría y
  la cronología de forma irrefutable.

---

## Programa de Early Adopters

Para los proveedores de modelos de IA que se interesen durante la fase
de especificación (antes de que exista regulación que lo exija):

| Categoría | Condiciones |
|-----------|------------|
| **Early Adopter** | 50% de descuento sobre tarifa estándar de licencia comercial |
| **Contribuidor activo** | Descuento adicional negociable por contribuciones al core |

El programa de Early Adopters reconoce que la integración temprana
beneficia a ambas partes: el proveedor obtiene ventaja competitiva
("nuestro modelo genera código Trenza-verificable"); Trenza obtiene
validación de mercado y feedback de integración.

> *Nota del Desarrollador Principal: Anthropic y Google tienen un
> descuento preferente del 50% si se interesan por Trenza. Son los
> proveedores de los modelos que co-diseñaron el lenguaje — justo es
> que tengan precio de familia.*

---

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|-----------|
| Un proveedor grande reimplementa la semántica sin licencia | La AGPL cubre el uso en servicios de red; la traza auditable en el repositorio público demuestra autoría y prioridad temporal |
| La regulación no llega | El negocio de licencia dual funciona sin regulación; la regulación es upside, no dependencia |
| Otro DSL verificable aparece primero | La traza auditable y la comunidad early-adopter crean switching costs; el diseño para LLMs es el diferenciador |
| El equipo es demasiado pequeño | El modelo no requiere escalar: pocos clientes grandes, spec que evoluciona lentamente, tests auto-generados |

---

## Próximos pasos

1. Registrar la propiedad intelectual (copyright de la especificación).
2. Publicar el repositorio como público con licencia AGPL-3.0.
3. Redactar el texto de la licencia comercial (consultar abogado IP).
4. Crear una landing page mínima para el proyecto.
5. Enviar la especificación a contactos en Anthropic y Google con nota
   sobre el programa de Early Adopters.
6. Evaluar la publicación de un paper técnico sobre verificabilidad de
   código generado por IA mediante DSLs restringidos.
