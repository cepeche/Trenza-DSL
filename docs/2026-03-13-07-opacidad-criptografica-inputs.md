# Opacidad criptográfica de entradas: seguridad por diseño en Trenza

**Fecha**: 13 de marzo de 2026
**Origen**: Desarrollador Principal (concepto), Claude (análisis técnico)
**Estado**: objetivo estratégico registrado — diseño pendiente

## Motivación

La sanitización de entradas de usuario es un problema endémico del desarrollo
de software. Décadas de SQL injection, XSS, command injection y variantes
demuestran que el enfoque de "filtrar lo malo" es frágil: siempre hay un
caso que se escapa, un encoding que no se contempló, una capa que asume que
otra ya sanitizó.

Nuestro desarrollador principal propone invertir el enfoque: **no sanitizar,
sino cifrar**. Si todo lo que entra desde la interfaz de usuario se cifra
inmediatamente en la frontera, lo que llega al interior del sistema es
ciphertext opaco. Ningún intérprete puede ejecutar código inyectado en algo
que es ruido binario.

Esto no es un argumento comercial. Es **seguridad por diseño**: la
imposibilidad estructural de una clase entera de vulnerabilidades.

## El principio

> Los datos que cruzan la frontera UI → sistema se encapsulan como
> **valores opacos tipados**, cifrados con una clave derivada del hash
> del propio sistema. Solo se descifran en los puntos que la especificación
> declara explícitamente.

Analogía: Rust distingue `OsString` (bytes opacos del SO) de `String`
(texto UTF-8 validado). No asumes que lo externo es válido hasta que
lo conviertes explícitamente. Trenza haría lo mismo, pero con una capa
criptográfica que hace la opacidad **infranqueable**, no solo convencional.

## Cómo encaja en Trenza

Trenza no es un framework web genérico. Las propiedades del DSL hacen
que este enfoque sea especialmente natural:

1. **Datos tipados y declarados**: `data.trz` define explícitamente cada
   tipo. El sistema sabe qué espera en cada punto.

2. **Transiciones deterministas**: no hay evaluación dinámica de strings.
   Los eventos son símbolos, no texto interpretado.

3. **Roles con tipos explícitos**: cada rol declara su tipo de dato.
   El cifrado puede ser **por campo tipado**, con esquemas de
   descifrado controlados por el verificador.

4. **Frontera UI → contexto bien definida**: los eventos son el único
   punto de entrada. El cifrado se aplica ahí, una sola vez.

## Flujo propuesto (preliminar)

```
[Usuario teclea "'; DROP TABLE..."]
        │
        ▼
┌─────────────────────┐
│  Frontera UI        │  cifrar(input, clave_sistema)
│  (capa de evento)   │  → 0xa7f3b2c1e9d8...
└─────────┬───────────┘
          │ valor opaco tipado
          ▼
┌─────────────────────┐
│  Contexto Trenza    │  El evento transporta el valor opaco.
│  (máquina estados)  │  Las transiciones operan sobre el evento,
│                     │  no sobre el contenido del dato.
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  Punto de consumo   │  descifrar(valor, clave_sistema)
│  (declarado en      │  → validar tipo → usar
│   especificación)   │  Solo aquí se interpreta el contenido.
└─────────────────────┘
```

El punto de consumo es explícito en la especificación: una función
external, un effect, un display. El verificador puede auditar que
**todo descifrado ocurre en puntos declarados**.

## Problemas abiertos

### Validación
No se puede validar formato (¿email válido? ¿longitud máxima?) sin
descifrar. Opciones:
- Validación en la frontera, antes del cifrado (sencillo pero reduce
  la protección a pre-cifrado)
- Validación en punto de consumo declarado (más seguro, el verificador
  lo audita)
- Esquema mixto: validaciones estructurales (longitud, tipo) en frontera;
  validaciones semánticas en punto de consumo

### Búsquedas y filtros
`WHERE nombre LIKE '%ana%'` no funciona sobre ciphertext. Opciones:
- Cifrado determinista para igualdad exacta (mismo input → mismo output)
- Índices sobre hashes truncados para búsquedas aproximadas
- Aceptar que las búsquedas requieren descifrado explícito (coherente
  con el modelo: buscar es un punto de consumo declarado)

### Key management
La clave derivada del hash del sistema debe ser accesible en runtime
pero protegida. El hash del sistema es verificable (está en el `.tzp`),
lo que da trazabilidad: si el sistema cambia, la clave cambia, y los
datos cifrados con la versión anterior son inaccesibles sin migración
explícita. Esto conecta directamente con el **ciclo de vida y versionado**
(ver `2026-03-13-05-ciclo-vida-sistemas.md`).

### Rendimiento
El cifrado/descifrado añade overhead. Para una app de UI con eventos
humanos (taps, tecleo), el coste es despreciable. Para sistemas con
alto throughput de datos, habría que evaluar.

## Relación con otros objetivos

- **Ciclo de vida** (`2026-03-13-05`): el cambio de versión del sistema
  implica cambio de clave. Los datos cifrados requieren migración
  explícita entre versiones. Esto es una *feature*, no un bug: fuerza
  la trazabilidad.

- **Verificador**: puede auditar estáticamente que todos los puntos de
  descifrado están declarados en la especificación. Si un contexto
  intenta usar el contenido de un valor opaco sin pasar por un punto
  de consumo declarado, es un error de verificación.

- **Reglas de verificación**: candidata a convertirse en la 7ª regla
  formal: **Opacidad** — todo dato externo se trata como opaco hasta
  su punto de consumo declarado.

## Prioridad

**Alta** como principio de diseño. La implementación criptográfica
concreta puede esperar, pero el modelo de "valores opacos tipados con
puntos de consumo declarados" debe informar el diseño de la gramática
desde ahora.

---

> *Nota del desarrollador principal*: "Si queremos que se nos tome en
> serio en privacidad y seguridad por diseño, esto es más que un
> argumento comercial."
