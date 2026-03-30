// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2 - TS)
// DO NOT EDIT — regenerate from .trz source
// Run with: npx vitest

import { describe, it, expect, vi } from 'vitest';
import { Contexto, System } from './CronometroPSP_out';
import type { Effects } from './CronometroPSP_out';
import { handle_item_nueva_actividad_tap, handle_item_historial_tap, handle_item_acerca_de_tap, handle_item_reset_tap, handle_overlay_tap, handle_boton_cerrar_tap, handle_campo_comentario_cambio, handle_campo_retroactivo_cambio, handle_boton_confirmar_tap, handle_boton_cancelar_tap, handle_campo_nombre_cambio, handle_selector_color_seleccion, handle_checkbox_permanente_cambio, handle_boton_guardar_tap, handle_campo_busqueda_icono_cambio, handle_selector_icono_seleccion, handle_checkbox_actividad_cambio, handle_boton_7dias_tap, handle_boton_30dias_tap, handle_boton_continuar_tap, handle_boton_exportar_csv_tap, handle_boton_atras_tap, handle_campo_confirmacion_cambio, handle_boton_ejecutar_tap, handle_boton_actividad_tap, handle_tarjeta_tipo_tap, handle_tarjeta_tarea_tap, handle_pestana_actividad_tap, handle_pestana_frecuentes_tap, handle_boton_edicion_tap, handle_boton_nuevo_tap, handle_boton_configuracion_tap, handle_display_timer_tap, handle_checkbox_sustituir_cambio } from './CronometroPSP_out';

const noOpEffects: Effects = {
    abrirAcercaDe: () => {},
    abrirCrearActividad: () => {},
    abrirCrearTarea: () => {},
    abrirEditarActividad: () => {},
    abrirEditarTarea: () => {},
    abrirHistorial: () => {},
    abrirMenuConfiguracion: () => {},
    abrirReset: () => {},
    activarEdicion: () => {},
    actualizarComentario: () => {},
    actualizarConfirmacion: () => {},
    actualizarGridVisible: () => {},
    actualizarNombre: () => {},
    actualizarNombreActividad: () => {},
    actualizarNombreNuevaActividad: () => {},
    actualizarNuevoNombre: () => {},
    actualizarRetroactivo: () => {},
    actualizar_actividad: () => {},
    actualizar_grid_visible: () => {},
    avanzarAFase2: () => {},
    avanzarAFase3: () => {},
    calcular_tiempo_transcurrido: () => {},
    cambiarA30Dias: () => {},
    cambiarA7Dias: () => {},
    cambiarPestana: () => {},
    cancelar: () => {},
    cargar_historial: () => {},
    cargar_tiempo_acumulado: () => {},
    cerrar: () => {},
    confirmarInicio: () => {},
    crear_actividad: () => {},
    crear_tipo_tarea: () => {},
    desactivarEdicion: () => {},
    descargar_csv: () => {},
    editar_tipo_tarea: () => {},
    ejecutarReset: () => {},
    elegirActividad: () => {},
    exportarCSV: () => {},
    filtrarIconos: () => {},
    filtrarIconosCrear: () => {},
    finalizar_sesion: () => {},
    guardarEdicion: () => {},
    guardarEdicionActividad: () => {},
    guardarNuevaActividad: () => {},
    guardarNuevaTarea: () => {},
    iniciarTarea: () => {},
    iniciar_sesion: () => {},
    marcarPermanente: () => {},
    marcarPermanenteNueva: () => {},
    marcarSustituir: () => {},
    obtener_sesion_activa: () => {},
    reset_datos: () => {},
    retrocederAFase1: () => {},
    retrocederAFase2: () => {},
    seleccionarColor: () => {},
    seleccionarColorNuevo: () => {},
    seleccionarIcono: () => {},
    seleccionarIconoNuevo: () => {},
    seleccionarTipoTarea: () => {},
    toggleActividadPermitida: () => {},
    toggleConservar: () => {},
    verificar_conexion: () => {},
};

function mockEffects(): Effects {
    return {
        abrirAcercaDe: vi.fn(),
        abrirCrearActividad: vi.fn(),
        abrirCrearTarea: vi.fn(),
        abrirEditarActividad: vi.fn(),
        abrirEditarTarea: vi.fn(),
        abrirHistorial: vi.fn(),
        abrirMenuConfiguracion: vi.fn(),
        abrirReset: vi.fn(),
        activarEdicion: vi.fn(),
        actualizarComentario: vi.fn(),
        actualizarConfirmacion: vi.fn(),
        actualizarGridVisible: vi.fn(),
        actualizarNombre: vi.fn(),
        actualizarNombreActividad: vi.fn(),
        actualizarNombreNuevaActividad: vi.fn(),
        actualizarNuevoNombre: vi.fn(),
        actualizarRetroactivo: vi.fn(),
        actualizar_actividad: vi.fn(),
        actualizar_grid_visible: vi.fn(),
        avanzarAFase2: vi.fn(),
        avanzarAFase3: vi.fn(),
        calcular_tiempo_transcurrido: vi.fn(),
        cambiarA30Dias: vi.fn(),
        cambiarA7Dias: vi.fn(),
        cambiarPestana: vi.fn(),
        cancelar: vi.fn(),
        cargar_historial: vi.fn(),
        cargar_tiempo_acumulado: vi.fn(),
        cerrar: vi.fn(),
        confirmarInicio: vi.fn(),
        crear_actividad: vi.fn(),
        crear_tipo_tarea: vi.fn(),
        desactivarEdicion: vi.fn(),
        descargar_csv: vi.fn(),
        editar_tipo_tarea: vi.fn(),
        ejecutarReset: vi.fn(),
        elegirActividad: vi.fn(),
        exportarCSV: vi.fn(),
        filtrarIconos: vi.fn(),
        filtrarIconosCrear: vi.fn(),
        finalizar_sesion: vi.fn(),
        guardarEdicion: vi.fn(),
        guardarEdicionActividad: vi.fn(),
        guardarNuevaActividad: vi.fn(),
        guardarNuevaTarea: vi.fn(),
        iniciarTarea: vi.fn(),
        iniciar_sesion: vi.fn(),
        marcarPermanente: vi.fn(),
        marcarPermanenteNueva: vi.fn(),
        marcarSustituir: vi.fn(),
        obtener_sesion_activa: vi.fn(),
        reset_datos: vi.fn(),
        retrocederAFase1: vi.fn(),
        retrocederAFase2: vi.fn(),
        seleccionarColor: vi.fn(),
        seleccionarColorNuevo: vi.fn(),
        seleccionarIcono: vi.fn(),
        seleccionarIconoNuevo: vi.fn(),
        seleccionarTipoTarea: vi.fn(),
        toggleActividadPermitida: vi.fn(),
        toggleConservar: vi.fn(),
        verificar_conexion: vi.fn(),
    } as unknown as Effects;
}

describe('Transitions', () => {
    it('MenuConfiguracion on abrirCrearActividad → ModalCrearActividad', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirCrearActividad');
        expect(sys.state).toBe(Contexto.ModalCrearActividad);
    });

    it('MenuConfiguracion on abrirHistorial → ModalHistorial', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirHistorial');
        expect(sys.state).toBe(Contexto.ModalHistorial);
    });

    it('MenuConfiguracion on abrirAcercaDe → ModalAcercaDe', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirAcercaDe');
        expect(sys.state).toBe(Contexto.ModalAcercaDe);
    });

    it('MenuConfiguracion on abrirReset → ModalReset', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirReset');
        expect(sys.state).toBe(Contexto.ModalReset);
    });

    it('MenuConfiguracion on cerrar → [close_overlay]', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalAcercaDe on cerrar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalAcercaDe, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalComentario on confirmarInicio → [close_overlay]', () => {
        const sys = new System(Contexto.ModalComentario, noOpEffects);
        sys.handleEvent('confirmarInicio');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalComentario on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalComentario, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalCrearActividad on guardarNuevaActividad → [close_overlay]', () => {
        const sys = new System(Contexto.ModalCrearActividad, noOpEffects);
        sys.handleEvent('guardarNuevaActividad');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalCrearActividad on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalCrearActividad, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalCrearTarea on guardarNuevaTarea → [close_overlay]', () => {
        const sys = new System(Contexto.ModalCrearTarea, noOpEffects);
        sys.handleEvent('guardarNuevaTarea');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalCrearTarea on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalCrearTarea, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalEditarActividad on guardarEdicionActividad → [close_overlay]', () => {
        const sys = new System(Contexto.ModalEditarActividad, noOpEffects);
        sys.handleEvent('guardarEdicionActividad');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalEditarActividad on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalEditarActividad, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalEditarTarea on guardarEdicion → [close_overlay]', () => {
        const sys = new System(Contexto.ModalEditarTarea, noOpEffects);
        sys.handleEvent('guardarEdicion');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalEditarTarea on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalEditarTarea, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModalHistorial on iniciar → Historial7Dias', () => {
        const sys = new System(Contexto.ModalHistorial, noOpEffects);
        sys.handleEvent('iniciar');
        expect(sys.state).toBe(Contexto.Historial7Dias);
    });

    it('ModalHistorial on cerrar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalHistorial, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('Historial7Dias on cambiarA30Dias → Historial30Dias', () => {
        const sys = new System(Contexto.Historial7Dias, noOpEffects);
        sys.handleEvent('cambiarA30Dias');
        expect(sys.state).toBe(Contexto.Historial30Dias);
    });

    it('Historial7Dias on cerrar → ModalHistorial', () => {
        const sys = new System(Contexto.Historial7Dias, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModalHistorial);
    });

    it('Historial30Dias on cambiarA7Dias → Historial7Dias', () => {
        const sys = new System(Contexto.Historial30Dias, noOpEffects);
        sys.handleEvent('cambiarA7Dias');
        expect(sys.state).toBe(Contexto.Historial7Dias);
    });

    it('Historial30Dias on cerrar → ModalHistorial', () => {
        const sys = new System(Contexto.Historial30Dias, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModalHistorial);
    });

    it('ModalReset on iniciar → ResetFase1', () => {
        const sys = new System(Contexto.ModalReset, noOpEffects);
        sys.handleEvent('iniciar');
        expect(sys.state).toBe(Contexto.ResetFase1);
    });

    it('ModalReset on cerrar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalReset, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ResetFase1 on avanzarAFase2 → ResetFase2', () => {
        const sys = new System(Contexto.ResetFase1, noOpEffects);
        sys.handleEvent('avanzarAFase2');
        expect(sys.state).toBe(Contexto.ResetFase2);
    });

    it('ResetFase1 on cerrar → ModalReset', () => {
        const sys = new System(Contexto.ResetFase1, noOpEffects);
        sys.handleEvent('cerrar');
        expect(sys.state).toBe(Contexto.ModalReset);
    });

    it('ResetFase2 on avanzarAFase3 → ResetFase3', () => {
        const sys = new System(Contexto.ResetFase2, noOpEffects);
        sys.handleEvent('avanzarAFase3');
        expect(sys.state).toBe(Contexto.ResetFase3);
    });

    it('ResetFase2 on retrocederAFase1 → ResetFase1', () => {
        const sys = new System(Contexto.ResetFase2, noOpEffects);
        sys.handleEvent('retrocederAFase1');
        expect(sys.state).toBe(Contexto.ResetFase1);
    });

    it('ResetFase3 on ejecutarReset → [close_overlay]', () => {
        const sys = new System(Contexto.ResetFase3, noOpEffects);
        sys.handleEvent('ejecutarReset');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ResetFase3 on retrocederAFase2 → ResetFase2', () => {
        const sys = new System(Contexto.ResetFase3, noOpEffects);
        sys.handleEvent('retrocederAFase2');
        expect(sys.state).toBe(Contexto.ResetFase2);
    });

    it('ModalSeleccionActividad on elegirActividad → ModalComentario', () => {
        const sys = new System(Contexto.ModalSeleccionActividad, noOpEffects);
        sys.handleEvent('elegirActividad');
        expect(sys.state).toBe(Contexto.ModalComentario);
    });

    it('ModalSeleccionActividad on cancelar → [close_overlay]', () => {
        const sys = new System(Contexto.ModalSeleccionActividad, noOpEffects);
        sys.handleEvent('cancelar');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModoEdicion on desactivarEdicion → ModoNormal', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('desactivarEdicion');
        expect(sys.state).toBe(Contexto.ModoNormal);
    });

    it('ModoEdicion on abrirEditarTarea → ModalEditarTarea', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirEditarTarea');
        expect(sys.state).toBe(Contexto.ModalEditarTarea);
    });

    it('ModoEdicion on abrirEditarActividad → ModalEditarActividad', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirEditarActividad');
        expect(sys.state).toBe(Contexto.ModalEditarActividad);
    });

    it('ModoEdicion on abrirCrearTarea → ModalCrearTarea', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirCrearTarea');
        expect(sys.state).toBe(Contexto.ModalCrearTarea);
    });

    it('ModoEdicion on abrirMenuConfiguracion → MenuConfiguracion', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirMenuConfiguracion');
        expect(sys.state).toBe(Contexto.MenuConfiguracion);
    });

    it('ModoNormal on activarEdicion → ModoEdicion', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('activarEdicion');
        expect(sys.state).toBe(Contexto.ModoEdicion);
    });

    it('ModoNormal on abrirCrearTarea → ModalCrearTarea', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('abrirCrearTarea');
        expect(sys.state).toBe(Contexto.ModalCrearTarea);
    });

    it('ModoNormal on abrirMenuConfiguracion → MenuConfiguracion', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('abrirMenuConfiguracion');
        expect(sys.state).toBe(Contexto.MenuConfiguracion);
    });

    it('ModoNormal on iniciarTarea → SesionActiva', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('iniciarTarea');
        expect(sys.state).toBe(Contexto.SesionActiva);
    });

    it('ModoNormal on seleccionarTipoTarea → ModalComentario', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('seleccionarTipoTarea');
        expect(sys.state).toBe(Contexto.ModalComentario);
    });

    it('ModoNormal on elegirActividad → ModalSeleccionActividad', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('elegirActividad');
        expect(sys.state).toBe(Contexto.ModalSeleccionActividad);
    });

});

describe('Overlay Stack', () => {
    it('opens ModalCrearActividad from MenuConfiguracion and returns on guardarNuevaActividad', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirCrearActividad'); // open overlay
        expect(sys.state).toBe(Contexto.ModalCrearActividad);
        sys.handleEvent('guardarNuevaActividad'); // close overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion); // restored
    });

    it('opens ModalHistorial from MenuConfiguracion and returns on cerrar', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirHistorial'); // open overlay
        expect(sys.state).toBe(Contexto.ModalHistorial);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion); // restored
    });

    it('opens ModalAcercaDe from MenuConfiguracion and returns on cerrar', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirAcercaDe'); // open overlay
        expect(sys.state).toBe(Contexto.ModalAcercaDe);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion); // restored
    });

    it('opens ModalReset from MenuConfiguracion and returns on cerrar', () => {
        const sys = new System(Contexto.MenuConfiguracion, noOpEffects);
        sys.handleEvent('abrirReset'); // open overlay
        expect(sys.state).toBe(Contexto.ModalReset);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion); // restored
    });

    it('opens ModalHistorial from Historial7Dias and returns on cerrar', () => {
        const sys = new System(Contexto.Historial7Dias, noOpEffects);
        sys.handleEvent('cerrar'); // open overlay
        expect(sys.state).toBe(Contexto.ModalHistorial);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.Historial7Dias); // restored
    });

    it('opens ModalHistorial from Historial30Dias and returns on cerrar', () => {
        const sys = new System(Contexto.Historial30Dias, noOpEffects);
        sys.handleEvent('cerrar'); // open overlay
        expect(sys.state).toBe(Contexto.ModalHistorial);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.Historial30Dias); // restored
    });

    it('opens ModalReset from ResetFase1 and returns on cerrar', () => {
        const sys = new System(Contexto.ResetFase1, noOpEffects);
        sys.handleEvent('cerrar'); // open overlay
        expect(sys.state).toBe(Contexto.ModalReset);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.ResetFase1); // restored
    });

    it('opens ModalComentario from ModalSeleccionActividad and returns on confirmarInicio', () => {
        const sys = new System(Contexto.ModalSeleccionActividad, noOpEffects);
        sys.handleEvent('elegirActividad'); // open overlay
        expect(sys.state).toBe(Contexto.ModalComentario);
        sys.handleEvent('confirmarInicio'); // close overlay
        expect(sys.state).toBe(Contexto.ModalSeleccionActividad); // restored
    });

    it('opens ModalEditarTarea from ModoEdicion and returns on guardarEdicion', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirEditarTarea'); // open overlay
        expect(sys.state).toBe(Contexto.ModalEditarTarea);
        sys.handleEvent('guardarEdicion'); // close overlay
        expect(sys.state).toBe(Contexto.ModoEdicion); // restored
    });

    it('opens ModalEditarActividad from ModoEdicion and returns on guardarEdicionActividad', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirEditarActividad'); // open overlay
        expect(sys.state).toBe(Contexto.ModalEditarActividad);
        sys.handleEvent('guardarEdicionActividad'); // close overlay
        expect(sys.state).toBe(Contexto.ModoEdicion); // restored
    });

    it('opens ModalCrearTarea from ModoEdicion and returns on guardarNuevaTarea', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirCrearTarea'); // open overlay
        expect(sys.state).toBe(Contexto.ModalCrearTarea);
        sys.handleEvent('guardarNuevaTarea'); // close overlay
        expect(sys.state).toBe(Contexto.ModoEdicion); // restored
    });

    it('opens MenuConfiguracion from ModoEdicion and returns on cerrar', () => {
        const sys = new System(Contexto.ModoEdicion, noOpEffects);
        sys.handleEvent('abrirMenuConfiguracion'); // open overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.ModoEdicion); // restored
    });

    it('opens ModalCrearTarea from ModoNormal and returns on guardarNuevaTarea', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('abrirCrearTarea'); // open overlay
        expect(sys.state).toBe(Contexto.ModalCrearTarea);
        sys.handleEvent('guardarNuevaTarea'); // close overlay
        expect(sys.state).toBe(Contexto.ModoNormal); // restored
    });

    it('opens MenuConfiguracion from ModoNormal and returns on cerrar', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('abrirMenuConfiguracion'); // open overlay
        expect(sys.state).toBe(Contexto.MenuConfiguracion);
        sys.handleEvent('cerrar'); // close overlay
        expect(sys.state).toBe(Contexto.ModoNormal); // restored
    });

    it('opens ModalComentario from ModoNormal and returns on confirmarInicio', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('seleccionarTipoTarea'); // open overlay
        expect(sys.state).toBe(Contexto.ModalComentario);
        sys.handleEvent('confirmarInicio'); // close overlay
        expect(sys.state).toBe(Contexto.ModoNormal); // restored
    });

    it('opens ModalSeleccionActividad from ModoNormal and returns on cancelar', () => {
        const sys = new System(Contexto.ModoNormal, noOpEffects);
        sys.handleEvent('elegirActividad'); // open overlay
        expect(sys.state).toBe(Contexto.ModalSeleccionActividad);
        sys.handleEvent('cancelar'); // close overlay
        expect(sys.state).toBe(Contexto.ModoNormal); // restored
    });

});

describe('Handlers', () => {
    it('MenuConfiguracion item_nueva_actividad tap invokes abrirCrearActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_item_nueva_actividad_tap(Contexto.MenuConfiguracion, {} as ItemMenu, effects);
        expect(event).toBe('abrirCrearActividad');
        expect((effects as any).abrirCrearActividad).toHaveBeenCalled();
    });

    it('MenuConfiguracion item_historial tap invokes abrirHistorial and returns event', () => {
        const effects = mockEffects();
        const event = handle_item_historial_tap(Contexto.MenuConfiguracion, {} as ItemMenu, effects);
        expect(event).toBe('abrirHistorial');
        expect((effects as any).abrirHistorial).toHaveBeenCalled();
    });

    it('MenuConfiguracion item_acerca_de tap invokes abrirAcercaDe and returns event', () => {
        const effects = mockEffects();
        const event = handle_item_acerca_de_tap(Contexto.MenuConfiguracion, {} as ItemMenu, effects);
        expect(event).toBe('abrirAcercaDe');
        expect((effects as any).abrirAcercaDe).toHaveBeenCalled();
    });

    it('MenuConfiguracion item_reset tap invokes abrirReset and returns event', () => {
        const effects = mockEffects();
        const event = handle_item_reset_tap(Contexto.MenuConfiguracion, {} as ItemMenu, effects);
        expect(event).toBe('abrirReset');
        expect((effects as any).abrirReset).toHaveBeenCalled();
    });

    it('MenuConfiguracion overlay tap invokes cerrar and returns event', () => {
        const effects = mockEffects();
        const event = handle_overlay_tap(Contexto.MenuConfiguracion, {} as Boton, effects);
        expect(event).toBe('cerrar');
        expect((effects as any).cerrar).toHaveBeenCalled();
    });

    it('ModalAcercaDe boton_cerrar tap invokes cerrar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cerrar_tap(Contexto.ModalAcercaDe, {} as Boton, effects);
        expect(event).toBe('cerrar');
        expect((effects as any).cerrar).toHaveBeenCalled();
    });

    it('ModalComentario campo_comentario cambio invokes actualizarComentario and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_comentario_cambio(Contexto.ModalComentario, {} as CampoTexto, effects);
        expect(event).toBe('actualizarComentario');
        expect((effects as any).actualizarComentario).toHaveBeenCalled();
    });

    it('ModalComentario campo_retroactivo cambio invokes actualizarRetroactivo and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_retroactivo_cambio(Contexto.ModalComentario, {} as CampoNumerico, effects);
        expect(event).toBe('actualizarRetroactivo');
        expect((effects as any).actualizarRetroactivo).toHaveBeenCalled();
    });

    it('ModalComentario boton_confirmar tap invokes confirmarInicio and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_confirmar_tap(Contexto.ModalComentario, {} as Boton, effects);
        expect(event).toBe('confirmarInicio');
        expect((effects as any).confirmarInicio).toHaveBeenCalled();
    });

    it('ModalComentario boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalComentario, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModalCrearActividad campo_nombre cambio invokes actualizarNombreNuevaActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_nombre_cambio(Contexto.ModalCrearActividad, {} as CampoTexto, effects);
        expect(event).toBe('actualizarNombreNuevaActividad');
        expect((effects as any).actualizarNombreNuevaActividad).toHaveBeenCalled();
    });

    it('ModalCrearActividad selector_color seleccion invokes seleccionarColorNuevo and returns event', () => {
        const effects = mockEffects();
        const event = handle_selector_color_seleccion(Contexto.ModalCrearActividad, {} as SelectorColor, effects);
        expect(event).toBe('seleccionarColorNuevo');
        expect((effects as any).seleccionarColorNuevo).toHaveBeenCalled();
    });

    it('ModalCrearActividad checkbox_permanente cambio invokes marcarPermanenteNueva and returns event', () => {
        const effects = mockEffects();
        const event = handle_checkbox_permanente_cambio(Contexto.ModalCrearActividad, {} as Checkbox, effects);
        expect(event).toBe('marcarPermanenteNueva');
        expect((effects as any).marcarPermanenteNueva).toHaveBeenCalled();
    });

    it('ModalCrearActividad boton_guardar tap invokes guardarNuevaActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_guardar_tap(Contexto.ModalCrearActividad, {} as Boton, effects);
        expect(event).toBe('guardarNuevaActividad');
        expect((effects as any).guardarNuevaActividad).toHaveBeenCalled();
    });

    it('ModalCrearActividad boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalCrearActividad, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModalCrearTarea campo_nombre cambio invokes actualizarNuevoNombre and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_nombre_cambio(Contexto.ModalCrearTarea, {} as CampoTexto, effects);
        expect(event).toBe('actualizarNuevoNombre');
        expect((effects as any).actualizarNuevoNombre).toHaveBeenCalled();
    });

    it('ModalCrearTarea campo_busqueda_icono cambio invokes filtrarIconosCrear and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_busqueda_icono_cambio(Contexto.ModalCrearTarea, {} as CampoTexto, effects);
        expect(event).toBe('filtrarIconosCrear');
        expect((effects as any).filtrarIconosCrear).toHaveBeenCalled();
    });

    it('ModalCrearTarea selector_icono seleccion invokes seleccionarIconoNuevo and returns event', () => {
        const effects = mockEffects();
        const event = handle_selector_icono_seleccion(Contexto.ModalCrearTarea, {} as SelectorIcono, effects);
        expect(event).toBe('seleccionarIconoNuevo');
        expect((effects as any).seleccionarIconoNuevo).toHaveBeenCalled();
    });

    it('ModalCrearTarea checkbox_actividad cambio invokes toggleActividadPermitida and returns event', () => {
        const effects = mockEffects();
        const event = handle_checkbox_actividad_cambio(Contexto.ModalCrearTarea, {} as OpcionActividad, effects);
        expect(event).toBe('toggleActividadPermitida');
        expect((effects as any).toggleActividadPermitida).toHaveBeenCalled();
    });

    it('ModalCrearTarea boton_guardar tap invokes guardarNuevaTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_guardar_tap(Contexto.ModalCrearTarea, {} as Boton, effects);
        expect(event).toBe('guardarNuevaTarea');
        expect((effects as any).guardarNuevaTarea).toHaveBeenCalled();
    });

    it('ModalCrearTarea boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalCrearTarea, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModalEditarActividad campo_nombre cambio invokes actualizarNombreActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_nombre_cambio(Contexto.ModalEditarActividad, {} as CampoTexto, effects);
        expect(event).toBe('actualizarNombreActividad');
        expect((effects as any).actualizarNombreActividad).toHaveBeenCalled();
    });

    it('ModalEditarActividad selector_color seleccion invokes seleccionarColor and returns event', () => {
        const effects = mockEffects();
        const event = handle_selector_color_seleccion(Contexto.ModalEditarActividad, {} as SelectorColor, effects);
        expect(event).toBe('seleccionarColor');
        expect((effects as any).seleccionarColor).toHaveBeenCalled();
    });

    it('ModalEditarActividad checkbox_permanente cambio invokes marcarPermanente and returns event', () => {
        const effects = mockEffects();
        const event = handle_checkbox_permanente_cambio(Contexto.ModalEditarActividad, {} as Checkbox, effects);
        expect(event).toBe('marcarPermanente');
        expect((effects as any).marcarPermanente).toHaveBeenCalled();
    });

    it('ModalEditarActividad boton_guardar tap invokes guardarEdicionActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_guardar_tap(Contexto.ModalEditarActividad, {} as Boton, effects);
        expect(event).toBe('guardarEdicionActividad');
        expect((effects as any).guardarEdicionActividad).toHaveBeenCalled();
    });

    it('ModalEditarActividad boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalEditarActividad, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModalEditarTarea campo_nombre cambio invokes actualizarNombre and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_nombre_cambio(Contexto.ModalEditarTarea, {} as CampoTexto, effects);
        expect(event).toBe('actualizarNombre');
        expect((effects as any).actualizarNombre).toHaveBeenCalled();
    });

    it('ModalEditarTarea campo_busqueda_icono cambio invokes filtrarIconos and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_busqueda_icono_cambio(Contexto.ModalEditarTarea, {} as CampoTexto, effects);
        expect(event).toBe('filtrarIconos');
        expect((effects as any).filtrarIconos).toHaveBeenCalled();
    });

    it('ModalEditarTarea selector_icono seleccion invokes seleccionarIcono and returns event', () => {
        const effects = mockEffects();
        const event = handle_selector_icono_seleccion(Contexto.ModalEditarTarea, {} as SelectorIcono, effects);
        expect(event).toBe('seleccionarIcono');
        expect((effects as any).seleccionarIcono).toHaveBeenCalled();
    });

    it('ModalEditarTarea boton_guardar tap invokes guardarEdicion and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_guardar_tap(Contexto.ModalEditarTarea, {} as Boton, effects);
        expect(event).toBe('guardarEdicion');
        expect((effects as any).guardarEdicion).toHaveBeenCalled();
    });

    it('ModalEditarTarea boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalEditarTarea, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModalHistorial boton_cerrar tap invokes cerrar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cerrar_tap(Contexto.ModalHistorial, {} as Boton, effects);
        expect(event).toBe('cerrar');
        expect((effects as any).cerrar).toHaveBeenCalled();
    });

    it('Historial7Dias boton_7dias tap returns null (ignored)', () => {
        const effects = mockEffects();
        const event = handle_boton_7dias_tap(Contexto.Historial7Dias, {} as Boton, effects);
        expect(event).toBeNull();
    });

    it('Historial7Dias boton_30dias tap invokes cambiarA30Dias and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_30dias_tap(Contexto.Historial7Dias, {} as Boton, effects);
        expect(event).toBe('cambiarA30Dias');
        expect((effects as any).cambiarA30Dias).toHaveBeenCalled();
    });

    it('Historial30Dias boton_7dias tap invokes cambiarA7Dias and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_7dias_tap(Contexto.Historial30Dias, {} as Boton, effects);
        expect(event).toBe('cambiarA7Dias');
        expect((effects as any).cambiarA7Dias).toHaveBeenCalled();
    });

    it('Historial30Dias boton_30dias tap returns null (ignored)', () => {
        const effects = mockEffects();
        const event = handle_boton_30dias_tap(Contexto.Historial30Dias, {} as Boton, effects);
        expect(event).toBeNull();
    });

    it('ModalReset boton_cancelar tap invokes cerrar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalReset, {} as Boton, effects);
        expect(event).toBe('cerrar');
        expect((effects as any).cerrar).toHaveBeenCalled();
    });

    it('ResetFase1 boton_cancelar tap invokes cerrar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ResetFase1, {} as Boton, effects);
        expect(event).toBe('cerrar');
        expect((effects as any).cerrar).toHaveBeenCalled();
    });

    it('ResetFase1 boton_continuar tap invokes avanzarAFase2 and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_continuar_tap(Contexto.ResetFase1, {} as Boton, effects);
        expect(event).toBe('avanzarAFase2');
        expect((effects as any).avanzarAFase2).toHaveBeenCalled();
    });

    it('ResetFase1 boton_exportar_csv tap invokes exportarCSV and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_exportar_csv_tap(Contexto.ResetFase1, {} as Boton, effects);
        expect(event).toBe('exportarCSV');
        expect((effects as any).exportarCSV).toHaveBeenCalled();
    });

    it('ResetFase2 checkbox_actividad cambio invokes toggleConservar and returns event', () => {
        const effects = mockEffects();
        const event = handle_checkbox_actividad_cambio(Contexto.ResetFase2, {} as OpcionActividad, effects);
        expect(event).toBe('toggleConservar');
        expect((effects as any).toggleConservar).toHaveBeenCalled();
    });

    it('ResetFase2 boton_continuar tap invokes avanzarAFase3 and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_continuar_tap(Contexto.ResetFase2, {} as Boton, effects);
        expect(event).toBe('avanzarAFase3');
        expect((effects as any).avanzarAFase3).toHaveBeenCalled();
    });

    it('ResetFase2 boton_atras tap invokes retrocederAFase1 and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_atras_tap(Contexto.ResetFase2, {} as Boton, effects);
        expect(event).toBe('retrocederAFase1');
        expect((effects as any).retrocederAFase1).toHaveBeenCalled();
    });

    it('ResetFase3 campo_confirmacion cambio invokes actualizarConfirmacion and returns event', () => {
        const effects = mockEffects();
        const event = handle_campo_confirmacion_cambio(Contexto.ResetFase3, {} as CampoTexto, effects);
        expect(event).toBe('actualizarConfirmacion');
        expect((effects as any).actualizarConfirmacion).toHaveBeenCalled();
    });

    it('ResetFase3 boton_ejecutar tap invokes ejecutarReset and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_ejecutar_tap(Contexto.ResetFase3, {} as Boton, effects);
        expect(event).toBe('ejecutarReset');
        expect((effects as any).ejecutarReset).toHaveBeenCalled();
    });

    it('ResetFase3 boton_atras tap invokes retrocederAFase2 and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_atras_tap(Contexto.ResetFase3, {} as Boton, effects);
        expect(event).toBe('retrocederAFase2');
        expect((effects as any).retrocederAFase2).toHaveBeenCalled();
    });

    it('ModalSeleccionActividad boton_actividad tap invokes elegirActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_actividad_tap(Contexto.ModalSeleccionActividad, {} as Actividad, effects);
        expect(event).toBe('elegirActividad');
        expect((effects as any).elegirActividad).toHaveBeenCalled();
    });

    it('ModalSeleccionActividad boton_cancelar tap invokes cancelar and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_cancelar_tap(Contexto.ModalSeleccionActividad, {} as Boton, effects);
        expect(event).toBe('cancelar');
        expect((effects as any).cancelar).toHaveBeenCalled();
    });

    it('ModoEdicion tarjeta_tipo tap invokes abrirEditarTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_tarjeta_tipo_tap(Contexto.ModoEdicion, {} as TipoTarea, effects);
        expect(event).toBe('abrirEditarTarea');
        expect((effects as any).abrirEditarTarea).toHaveBeenCalled();
    });

    it('ModoEdicion tarjeta_tarea tap invokes abrirEditarTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_tarjeta_tarea_tap(Contexto.ModoEdicion, {} as Tarea, effects);
        expect(event).toBe('abrirEditarTarea');
        expect((effects as any).abrirEditarTarea).toHaveBeenCalled();
    });

    it('ModoEdicion pestana_actividad tap invokes abrirEditarActividad and returns event', () => {
        const effects = mockEffects();
        const event = handle_pestana_actividad_tap(Contexto.ModoEdicion, {} as Actividad, effects);
        expect(event).toBe('abrirEditarActividad');
        expect((effects as any).abrirEditarActividad).toHaveBeenCalled();
    });

    it('ModoEdicion pestana_frecuentes tap returns null (ignored)', () => {
        const effects = mockEffects();
        const event = handle_pestana_frecuentes_tap(Contexto.ModoEdicion, {} as Pestana, effects);
        expect(event).toBeNull();
    });

    it('ModoEdicion boton_edicion tap invokes desactivarEdicion and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_edicion_tap(Contexto.ModoEdicion, {} as Boton, effects);
        expect(event).toBe('desactivarEdicion');
        expect((effects as any).desactivarEdicion).toHaveBeenCalled();
    });

    it('ModoEdicion boton_nuevo tap invokes abrirCrearTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_nuevo_tap(Contexto.ModoEdicion, {} as Boton, effects);
        expect(event).toBe('abrirCrearTarea');
        expect((effects as any).abrirCrearTarea).toHaveBeenCalled();
    });

    it('ModoEdicion boton_configuracion tap invokes abrirMenuConfiguracion and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_configuracion_tap(Contexto.ModoEdicion, {} as Boton, effects);
        expect(event).toBe('abrirMenuConfiguracion');
        expect((effects as any).abrirMenuConfiguracion).toHaveBeenCalled();
    });

    it('ModoNormal tarjeta_tipo tap invokes seleccionarTipoTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_tarjeta_tipo_tap(Contexto.ModoNormal, {} as TipoTarea, effects);
        expect(event).toBe('seleccionarTipoTarea');
        expect((effects as any).seleccionarTipoTarea).toHaveBeenCalled();
    });

    it('ModoNormal tarjeta_tarea tap invokes iniciarTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_tarjeta_tarea_tap(Contexto.ModoNormal, {} as Tarea, effects);
        expect(event).toBe('iniciarTarea');
        expect((effects as any).iniciarTarea).toHaveBeenCalled();
    });

    it('ModoNormal pestana_actividad tap invokes cambiarPestana and returns event', () => {
        const effects = mockEffects();
        const event = handle_pestana_actividad_tap(Contexto.ModoNormal, {} as Actividad, effects);
        expect(event).toBe('cambiarPestana');
        expect((effects as any).cambiarPestana).toHaveBeenCalled();
    });

    it('ModoNormal pestana_frecuentes tap invokes cambiarPestana and returns event', () => {
        const effects = mockEffects();
        const event = handle_pestana_frecuentes_tap(Contexto.ModoNormal, {} as Pestana, effects);
        expect(event).toBe('cambiarPestana');
        expect((effects as any).cambiarPestana).toHaveBeenCalled();
    });

    it('ModoNormal boton_edicion tap invokes activarEdicion and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_edicion_tap(Contexto.ModoNormal, {} as Boton, effects);
        expect(event).toBe('activarEdicion');
        expect((effects as any).activarEdicion).toHaveBeenCalled();
    });

    it('ModoNormal boton_nuevo tap invokes abrirCrearTarea and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_nuevo_tap(Contexto.ModoNormal, {} as Boton, effects);
        expect(event).toBe('abrirCrearTarea');
        expect((effects as any).abrirCrearTarea).toHaveBeenCalled();
    });

    it('ModoNormal boton_configuracion tap invokes abrirMenuConfiguracion and returns event', () => {
        const effects = mockEffects();
        const event = handle_boton_configuracion_tap(Contexto.ModoNormal, {} as Boton, effects);
        expect(event).toBe('abrirMenuConfiguracion');
        expect((effects as any).abrirMenuConfiguracion).toHaveBeenCalled();
    });

    it('SesionActiva display_timer tap returns null (ignored)', () => {
        const effects = mockEffects();
        const event = handle_display_timer_tap(Contexto.SesionActiva, {} as Boton, effects);
        expect(event).toBeNull();
    });

});

describe('Exhaustiveness', () => {
    it('Contexto enum has all contexts', () => {
        expect(Object.keys(Contexto).length).toBe(18);
    });
});
