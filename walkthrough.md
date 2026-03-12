# Diagrama de Topología del AST Trenza

Aquí tienes la representación visual del AST parseado mediante el Prototipo en Python, estructurado usando StateDiagram-V2 de Mermaid. Puedes previsualizarlo aquí directamente en nuestra interfaz con soporte nativo.

```mermaid
stateDiagram-v2
    state CronometroPSP {
        [*] --> ModoNormal
        state ModoEdicion {
            note right of ModoEdicion
                tarjeta_tipo: TipoTarea
                tarjeta_tarea: Tarea
                pestana_actividad: Actividad
                pestana_frecuentes: Pestaña
                boton_edicion: Boton
                boton_nuevo: Boton
                boton_configuracion: Boton
            end note
        }
        state ModoNormal {
            note right of ModoNormal
                tarjeta_tipo: TipoTarea
                tarjeta_tarea: Tarea
                pestana_actividad: Actividad
                pestana_frecuentes: Pestaña
                boton_edicion: Boton
                boton_nuevo: Boton
                boton_configuracion: Boton
            end note
        }
    }
    state CO_CronometroPSP {
        note left of CO_CronometroPSP: Concurrent Contexts
        state SesionActiva {
            note right of SesionActiva
                display_timer: Boton
                checkbox_sustituir: Checkbox
            end note
        }
    }
    state OV_CronometroPSP {
        note left of OV_CronometroPSP: Overlay Contexts
        state MenuConfiguracion {
            note right of MenuConfiguracion
                item_nueva_actividad: ItemMenu
                item_historial: ItemMenu
                item_acerca_de: ItemMenu
                item_reset: ItemMenu
                overlay: Boton
            end note
        }
        state ModalComentario {
            note right of ModalComentario
                campo_comentario: CampoTexto
                campo_retroactivo: CampoNumerico
                boton_confirmar: Boton
                boton_cancelar: Boton
            end note
        }
        state ModalSeleccionActividad {
            note right of ModalSeleccionActividad
                boton_actividad: Actividad
                boton_cancelar: Boton
            end note
        }
        state ModalCrearTarea {
            note right of ModalCrearTarea
                campo_nombre: CampoTexto
                campo_busqueda_icono: CampoTexto
                selector_icono: SelectorIcono
                checkbox_actividad: Actividad
                boton_guardar: Boton
                boton_cancelar: Boton
            end note
        }
        state ModalEditarTarea {
            note right of ModalEditarTarea
                campo_nombre: CampoTexto
                campo_busqueda_icono: CampoTexto
                selector_icono: SelectorIcono
                boton_guardar: Boton
                boton_cancelar: Boton
            end note
        }
        state ModalEditarActividad {
            note right of ModalEditarActividad
                campo_nombre: CampoTexto
                selector_color: SelectorColor
                checkbox_permanente: Checkbox
                boton_guardar: Boton
                boton_cancelar: Boton
            end note
        }
        state ModalCrearActividad {
            note right of ModalCrearActividad
                campo_nombre: CampoTexto
                selector_color: SelectorColor
                checkbox_permanente: Checkbox
                boton_guardar: Boton
                boton_cancelar: Boton
            end note
        }
        state ModalHistorial {
            note right of ModalHistorial
                boton_cerrar: Boton
            end note
            state Historial7Dias {
                note right of Historial7Dias
                    boton_7dias: Boton
                    boton_30dias: Boton
                end note
                state Historial30Dias {
                    note right of Historial30Dias
                        boton_7dias: Boton
                        boton_30dias: Boton
                    end note
                }
            }
        }
        state ModalReset {
            note right of ModalReset
                boton_cancelar: Boton
            end note
            state ResetFase1 {
                note right of ResetFase1
                    boton_continuar: Boton
                    boton_exportar_csv: Boton
                end note
                state ResetFase2 {
                    note right of ResetFase2
                        checkbox_actividad: OpcionActividad
                        boton_continuar: Boton
                        boton_atras: Boton
                    end note
                    state ResetFase3 {
                        note right of ResetFase3
                            campo_confirmacion: CampoTexto
                            boton_ejecutar: Boton
                            boton_atras: Boton
                        end note
                    }
                }
            }
        }
        state ModalAcercaDe {
            note right of ModalAcercaDe
                boton_cerrar: Boton
            end note
        }
    }
    ModoEdicion --> ModoNormal : desactivarEdicion
    ModoEdicion --> ModalEditarTarea : abrirEditarTarea
    ModoEdicion --> ModalEditarActividad : abrirEditarActividad
    ModoEdicion --> ModalCrearTarea : abrirCrearTarea
    ModoEdicion --> MenuConfiguracion : abrirMenuConfiguracion
    ModoNormal --> ModoEdicion : activarEdicion
    ModoNormal --> ModalCrearTarea : abrirCrearTarea
    ModoNormal --> MenuConfiguracion : abrirMenuConfiguracion
    ModoNormal --> ModalComentario : seleccionarTipoTarea
    SesionActiva --> [desactivar] : sesionFinalizada
    MenuConfiguracion --> ModalCrearActividad : abrirCrearActividad
    MenuConfiguracion --> ModalHistorial : abrirHistorial
    MenuConfiguracion --> ModalAcercaDe : abrirAcercaDe
    MenuConfiguracion --> ModalReset : abrirReset
    MenuConfiguracion --> [cerrar_overlay] : cerrar
    ModalComentario --> [cerrar_overlay] : confirmarInicio
    ModalComentario --> [cerrar_overlay] : cancelar
    ModalSeleccionActividad --> ModalComentario : elegirActividad
    ModalSeleccionActividad --> [cerrar_overlay] : cancelar
    ModalCrearTarea --> [cerrar_overlay] : guardarNuevaTarea
    ModalCrearTarea --> [cerrar_overlay] : cancelar
    ModalEditarTarea --> [cerrar_overlay] : guardarEdicion
    ModalEditarTarea --> [cerrar_overlay] : cancelar
    ModalEditarActividad --> [cerrar_overlay] : guardarEdicionActividad
    ModalEditarActividad --> [cerrar_overlay] : cancelar
    ModalCrearActividad --> [cerrar_overlay] : guardarNuevaActividad
    ModalCrearActividad --> [cerrar_overlay] : cancelar
    ModalHistorial --> [cerrar_overlay] : cerrar
    ModalHistorial --> Historial30Dias : cambiarA30Dias
    ModalHistorial --> ModalHistorial : cerrar
    Historial7Dias --> Historial30Dias : cambiarA30Dias
    Historial7Dias --> ModalHistorial : cerrar
    Historial7Dias --> Historial7Dias : cambiarA7Dias
    Historial7Dias --> ModalHistorial : cerrar
    Historial30Dias --> Historial7Dias : cambiarA7Dias
    Historial30Dias --> ModalHistorial : cerrar
    ModalReset --> [cerrar_overlay] : cerrar
    ModalReset --> ResetFase2 : avanzarAFase2
    ModalReset --> ModalReset : cerrar
    ResetFase1 --> ResetFase2 : avanzarAFase2
    ResetFase1 --> ModalReset : cerrar
    ResetFase1 --> ResetFase3 : avanzarAFase3
    ResetFase1 --> ResetFase1 : retrocederAFase1
    ResetFase2 --> ResetFase3 : avanzarAFase3
    ResetFase2 --> ResetFase1 : retrocederAFase1
    ResetFase2 --> [cerrar_overlay] : ejecutarReset
    ResetFase2 --> ResetFase2 : retrocederAFase2
    ResetFase3 --> [cerrar_overlay] : ejecutarReset
    ResetFase3 --> ResetFase2 : retrocederAFase2
    ModalAcercaDe --> [cerrar_overlay] : cerrar
```
