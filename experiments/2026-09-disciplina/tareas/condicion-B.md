**Tu entorno.** El comportamiento de la aplicación está especificado en
Trenza, un lenguaje de especificación de máquinas de estados por roles, en
el archivo `{WORKDIR}/cronometro.trz`. La implementación se genera
automáticamente a partir de él; tú solo cambias el `.trz`. La guía del
lenguaje está en `{WORKDIR}/GUIA-TRENZA.md`. Puedes verificar la
especificación con `{CLI} check {WORKDIR}/cronometro.trz`. Trabaja
**exclusivamente** dentro de `{WORKDIR}`: no leas ni escribas fuera de ese
directorio (salvo para ejecutar `{CLI}`).
