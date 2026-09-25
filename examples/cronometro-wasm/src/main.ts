// main.ts — Integración Trenza ↔ DOM.
//
// Arquitectura:
// - Los 10 overlays son proyección directa del estado (overlays.ts, Fase 2).
// - Los botones usan data-event / data-payload-* para cableo declarativo.
// - Las funciones de render (render.ts) leen storage y escriben el DOM.
// - formState captura inputs antes de guardar.
// - sesionEnCurso mantiene la ventana inicio↔fin (ver R-trz2 en memo).

import { Contexto } from './CronometroPSP_out';
import type { Effects } from './CronometroPSP_out';
import { createTrenzaSystem } from './snapshot-bridge';
import {
  makeDispatchWithSync,
  makeOverlayEffectStubs,
  syncOverlayVisibility,
  wireGlobalCloseHandlers,
} from './overlays';
import {
  listTareas,
  createTarea,
  listActividades,
  appendSesion,
  clearAll,
} from './storage';
import {
  renderTasksGrid,
  renderActivityButtons,
  renderHistorial,
  renderTotalToday,
  renderActiveTimer,
  renderResetActivities,
} from './render';

async function run() {
  // ── Estado en memoria ────────────────────────────────────────────────────
  const formState: Record<string, unknown> = {};

  // R-trz2: ventana inicio↔fin de sesión. Se captura en iniciar_sesion,
  // se persiste en parar_sesion (o por fallback de detección de estado).
  let sesionEnCurso: {
    tareaId: string;
    actividadId: string;
    inicio: number;
    nombre: string;
  } | null = null;

  let timerInterval: ReturnType<typeof setInterval> | null = null;
  let timerSeconds = 0;

  // ── Helpers ──────────────────────────────────────────────────────────────

  function flushSesion(comentario: string | null = null): void {
    if (!sesionEnCurso) return;
    appendSesion({
      tareaId: sesionEnCurso.tareaId,
      actividadId: sesionEnCurso.actividadId,
      inicio: sesionEnCurso.inicio,
      fin: Date.now(),
      comentario,
    });
    if (timerInterval) { clearInterval(timerInterval); timerInterval = null; }
    sesionEnCurso = null;
    timerSeconds = 0;
    renderActiveTimer('Ninguna tarea activa', 0);
    renderTotalToday();
  }

  function extractPayload(el: HTMLElement): Record<string, string> {
    const payload: Record<string, string> = {};
    for (const attr of el.attributes) {
      if (attr.name.startsWith('data-payload-')) {
        const key = attr.name
          .replace('data-payload-', '')
          .replace(/-([a-z])/g, (_: string, c: string) => c.toUpperCase());
        payload[key] = attr.value;
      }
    }
    return payload;
  }

  // ── Efectos ──────────────────────────────────────────────────────────────

  // Construidos antes de system; las referencias a system/dispatch/safeDispatch
  // se resuelven en closure cuando los efectos se invocan (siempre posterior
  // a la asignación de system).
  const effectsObj: Partial<Effects> & { parar_sesion?: () => void } = {

    // Regla 1: overlay stubs (visibilidad proyectada por estado)
    ...makeOverlayEffectStubs(),

    // Regla 2: transiciones puras → no-op (el estado actualiza la UI)
    activarEdicion:    () => {},
    desactivarEdicion: () => {},
    avanzarAFase2: () => {
      // El estado entra en ResetFase1→ResetFase2. Mostramos el paso 2 del modal.
      document.getElementById('resetStep1')?.style.setProperty('display', 'none');
      document.getElementById('resetStep2')?.style.setProperty('display', 'block');
      document.getElementById('resetStep3')?.style.setProperty('display', 'none');
      renderResetActivities();
    },
    avanzarAFase3: () => {
      document.getElementById('resetStep1')?.style.setProperty('display', 'none');
      document.getElementById('resetStep2')?.style.setProperty('display', 'none');
      document.getElementById('resetStep3')?.style.setProperty('display', 'block');
      const btnReset = document.getElementById('btnReset');
      if (btnReset) btnReset.textContent = 'ELIMINAR TODO';
    },
    retrocederAFase1: () => {
      document.getElementById('resetStep1')?.style.setProperty('display', 'block');
      document.getElementById('resetStep2')?.style.setProperty('display', 'none');
      document.getElementById('resetStep3')?.style.setProperty('display', 'none');
    },
    retrocederAFase2: () => {
      document.getElementById('resetStep1')?.style.setProperty('display', 'none');
      document.getElementById('resetStep2')?.style.setProperty('display', 'block');
      document.getElementById('resetStep3')?.style.setProperty('display', 'none');
    },
    cambiarA7Dias:  () => { renderHistorial(system); },
    cambiarA30Dias: () => { renderHistorial(system); },

    // Regla 3: input binding → formState
    actualizarComentario:          (v) => { formState.comentario = v; },
    actualizarConfirmacion:        (v) => { formState.confirmacion = v; },
    actualizarGridVisible:         () => { renderTasksGrid(safeDispatch); },
    actualizarNombre:              (v) => { formState.nombre = v; },
    actualizarNombreActividad:     (v) => { formState.nombreActividad = v; },
    actualizarNombreNuevaActividad:(v) => { formState.nombreNuevaActividad = v; },
    actualizarNuevoNombre:         (v) => { formState.nuevoNombre = v; },
    actualizarRetroactivo:         (v) => { formState.retroactivo = Number(v) || 0; },
    actualizar_actividad:          () => {},
    cambiarPestana:                () => {},
    filtrarIconos:                 () => {},
    filtrarIconosCrear:            () => {},
    marcarPermanente:              (v) => { formState.permanente = v; },
    marcarPermanenteNueva:         (v) => { formState.permanenteNueva = v; },
    marcarSustituir:               (v) => { formState.sustituir = v; },
    seleccionarColor:              (v) => { formState.color = v; },
    seleccionarColorNuevo:         (v) => { formState.colorNuevo = v; },
    seleccionarIcono:              (v) => { formState.icono = v; },
    seleccionarIconoNuevo:         (v) => { formState.iconoNuevo = v; },
    seleccionarTipoTarea:          (v) => { formState.tipoTarea = v; },
    toggleActividadPermitida:      () => {},
    toggleConservar:               () => {},

    // Regla 4: guardar → storage
    guardarNuevaTarea: () => {
      const nombre = (
        (document.getElementById('newTaskName') as HTMLInputElement | null)?.value.trim()
        || String(formState.nombre ?? '')
        || 'Nueva Tarea'
      );
      const icono = String(formState.iconoNuevo ?? '📋');
      const actividades = listActividades();
      createTarea({ nombre, icono, actividadIds: actividades.map(a => a.id) });
      formState.nombre = '';
      formState.iconoNuevo = '';
      renderTasksGrid(safeDispatch);
    },
    guardarNuevaActividad: () => {
      // FLAG: createActividad no está en el API de storage (Brief B).
      // No implementado. Ver memo hallazgo R-storage1.
      console.warn('[FLAG R-storage1] guardarNuevaActividad: createActividad no disponible en storage.');
    },
    guardarEdicion: () => {
      // FLAG: updateTarea no está en el API de storage.
      // Stub hasta siguiente sprint. Ver memo.
      console.warn('[FLAG R-storage2] guardarEdicion: updateTarea no disponible en storage.');
      renderTasksGrid(safeDispatch);
    },
    guardarEdicionActividad: () => {
      console.warn('[FLAG R-storage3] guardarEdicionActividad: updateActividad no disponible.');
    },
    editar_tipo_tarea:  () => {},
    crear_actividad:    () => {},
    crear_tipo_tarea:   () => {},

    // Regla 5: cargar → storage + render
    cargar_historial:       () => { renderHistorial(system); },
    cargar_tiempo_acumulado: () => { renderTotalToday(); },

    // Regla 6: reset → clearAll + render
    ejecutarReset: () => {
      clearAll();
      if (sesionEnCurso) {
        if (timerInterval) { clearInterval(timerInterval); timerInterval = null; }
        sesionEnCurso = null;
        timerSeconds = 0;
      }
      // Restaurar pasos del modal a Fase 1 para próxima vez
      document.getElementById('resetStep1')?.style.setProperty('display', 'block');
      document.getElementById('resetStep2')?.style.setProperty('display', 'none');
      document.getElementById('resetStep3')?.style.setProperty('display', 'none');
      const btnReset = document.getElementById('btnReset');
      if (btnReset) btnReset.textContent = 'Continuar';
      const resetInput = document.getElementById('resetConfirmInput') as HTMLInputElement | null;
      if (resetInput) resetInput.value = '';
      renderActiveTimer('Ninguna tarea activa', 0);
      renderTasksGrid(safeDispatch);
      renderTotalToday();
    },
    reset_datos: () => {
      clearAll();
      renderTasksGrid(safeDispatch);
      renderTotalToday();
    },

    // Regla 7: externos no implementados → stub con log
    descargar_csv:     () => { console.log('[stub] descargar_csv — fuera de alcance del demo'); },
    exportarCSV:       () => { console.log('[stub] exportarCSV — fuera de alcance del demo'); },
    verificar_conexion: () => { console.log('[stub] verificar_conexion — fuera de alcance del demo'); },

    // Regla 8: sesión
    iniciarTarea: (tareaId) => {
      // Almacena el tareaId para el modal de selección de actividad.
      formState.currentTareaId = tareaId ?? listTareas()[0]?.id ?? '';
    },

    iniciar_sesion: (tareaId, comentario, retroactivo, _sustituir) => {
      const resolvedTareaId = tareaId ?? formState.currentTareaId ?? listTareas()[0]?.id ?? '';
      const resolvedActividadId = (formState.currentActividadId as string | undefined)
        ?? listActividades()[0]?.id ?? '';
      const retroMs = (Number(retroactivo ?? formState.retroactivo) || 0) * 60_000;

      sesionEnCurso = {
        tareaId: resolvedTareaId,
        actividadId: resolvedActividadId,
        inicio: Date.now() - retroMs,
        nombre: listTareas().find(t => t.id === resolvedTareaId)?.nombre ?? 'Tarea',
      };
      timerSeconds = Math.floor((Date.now() - sesionEnCurso.inicio) / 1000);
      renderActiveTimer(sesionEnCurso.nombre, timerSeconds);

      if (timerInterval) clearInterval(timerInterval);
      timerInterval = setInterval(() => {
        timerSeconds++;
        renderActiveTimer(sesionEnCurso?.nombre ?? '', timerSeconds);
      }, 1000);

      // Guardar comentario en formState para flushSesion
      formState.ultimoComentario = comentario ?? null;
    },

    // FLAG R-trz1: parar_sesion declarado en .trz línea 1420 pero ausente
    // del Effects interface generado. Registrado defensivamente.
    parar_sesion: () => {
      flushSesion(formState.ultimoComentario as string | null ?? null);
    },

    elegirActividad: (actividadId) => {
      formState.currentActividadId = actividadId;
    },

    confirmarInicio: () => {
      // El comentario ya está en formState.comentario vía actualizarComentario.
      // El .trz hace la transición; iniciar_sesion se disparará desde el
      // interprete con los valores acumulados.
    },

    calcular_tiempo_transcurrido: (_inicio) => {
      // Timer gestionado vía setInterval. Este efecto es informativo.
    },
  };

  // ── Sistema ───────────────────────────────────────────────────────────────

  // Per-spec WASM shim: SystemWasm is the deterministic engine emitted by
  // trenza-cli; createTrenzaSystem wraps it with the legacy interface
  // (current_state, concurrent_states, dispatch) and re-routes recorded
  // effect calls back to effectsObj.
  const system = await createTrenzaSystem(
    effectsObj as unknown as Record<string, (...args: unknown[]) => void>,
  );
  const baseDispatch = makeDispatchWithSync(system);

  // safeDispatch: wraps baseDispatch con hooks post-dispatch:
  // 1. Fallback R-trz2: si salimos de SesionActiva sin parar_sesion, flush.
  // 2. Post-render para modales que necesitan datos dinámicos.
  function safeDispatch(event: string, payload: unknown = {}): void {
    const prevStates = new Set(system.concurrent_states);

    baseDispatch(event, payload);

    const nextStates = system.concurrent_states;

    // R-trz2 fallback: detectar salida de SesionActiva
    const wasActive = prevStates.has(Contexto.SesionActiva);
    const isActive  = nextStates.has(Contexto.SesionActiva);
    if (wasActive && !isActive && sesionEnCurso) {
      flushSesion(formState.ultimoComentario as string | null ?? null);
    }

    // Render dinámico post-dispatch
    if (!prevStates.has(Contexto.ModalSeleccionActividad) &&
         nextStates.has(Contexto.ModalSeleccionActividad)) {
      renderActivityButtons(
        String(formState.currentTareaId ?? listTareas()[0]?.id ?? ''),
        safeDispatch,
      );
    }

    if (!prevStates.has(Contexto.ModalHistorial) &&
         nextStates.has(Contexto.ModalHistorial)) {
      renderHistorial(system);
    }

    if (nextStates.has(Contexto.ModoEdicion) !== prevStates.has(Contexto.ModoEdicion)) {
      renderTasksGrid(safeDispatch);
    }

    // Reset 3-fases: project the current sub-context to which step DOM is
    // visible and which label the action button shows. The .trz declares the
    // transitions as pure state changes (no event-effects), so we mirror the
    // top of the overlay stack here. This avoids declaring DOM detail in the
    // spec while keeping the visual flow tied 1:1 to the runtime model.
    const top = system.current_state;
    const step1 = document.getElementById('resetStep1');
    const step2 = document.getElementById('resetStep2');
    const step3 = document.getElementById('resetStep3');
    const btnReset = document.getElementById('btnReset');
    if (top === Contexto.ResetFase1) {
      step1?.style.setProperty('display', 'block');
      step2?.style.setProperty('display', 'none');
      step3?.style.setProperty('display', 'none');
      if (btnReset) btnReset.textContent = 'Continuar';
    } else if (top === Contexto.ResetFase2) {
      step1?.style.setProperty('display', 'none');
      step2?.style.setProperty('display', 'block');
      step3?.style.setProperty('display', 'none');
      if (btnReset) btnReset.textContent = 'Continuar';
      renderResetActivities();
    } else if (top === Contexto.ResetFase3) {
      step1?.style.setProperty('display', 'none');
      step2?.style.setProperty('display', 'none');
      step3?.style.setProperty('display', 'block');
      if (btnReset) btnReset.textContent = 'ELIMINAR TODO';
    }
  }

  wireGlobalCloseHandlers(safeDispatch);
  syncOverlayVisibility(system);

  // Render inicial
  renderTasksGrid(safeDispatch);
  renderTotalToday();
  renderActiveTimer('Ninguna tarea activa', 0);

  // ── Cableo declarativo ───────────────────────────────────────────────────

  // Delegación global de clics en [data-event]
  document.addEventListener('click', (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>('[data-event]');
    if (!el) return;
    // task-cards tienen su propio listener con stopPropagation;
    // este handler cubre el resto.
    const event = el.dataset.event!;
    const payload = extractPayload(el);
    safeDispatch(event, payload);
  });

  // Delegación de inputs en [data-input-event]
  document.addEventListener('input', (e) => {
    const el = e.target as HTMLInputElement;
    const event = el.dataset.inputEvent;
    if (!event) return;
    safeDispatch(event, { valor: el.value });
  });

  // Botón modo edición: toggle según estado actual
  document.getElementById('editModeButton')?.addEventListener('click', () => {
    const inEdicion = system.concurrent_states.has(Contexto.ModoEdicion)
      || system.current_state === Contexto.ModoEdicion;
    safeDispatch(inEdicion ? 'desactivarEdicion' : 'activarEdicion');
  });

  // Botón Continuar/Confirmar del modal Reset (3 fases)
  document.getElementById('btnReset')?.addEventListener('click', () => {
    const states = system.concurrent_states;
    if (states.has(Contexto.ResetFase1) || system.current_state === Contexto.ResetFase1 as unknown) {
      safeDispatch('avanzarAFase2');
    } else if (states.has(Contexto.ResetFase2) || system.current_state === Contexto.ResetFase2 as unknown) {
      safeDispatch('avanzarAFase3');
    } else if (states.has(Contexto.ResetFase3) || system.current_state === Contexto.ResetFase3 as unknown) {
      const input = (document.getElementById('resetConfirmInput') as HTMLInputElement | null)?.value;
      if (input === 'BORRAR') {
        safeDispatch('ejecutarReset');
      }
    }
  });

  // Timer activo: click en el header para parar la sesión
  document.querySelector('.active-timer')?.addEventListener('click', () => {
    if (sesionEnCurso) {
      safeDispatch('terminarSesion');
    }
  });

  // Cierre del menú de settings al hacer click fuera. Defensa contra
  // interferencia con modales apilados:
  //  - si el target está DENTRO de settingsMenu (un item del menú): no
  //    disparamos cerrar — el item ya tiene su propio handler y un cerrar
  //    aquí cancelaría la transición que el item acaba de iniciar.
  //  - si el target está dentro de cualquier .modal-overlay (un modal
  //    abierto sobre settings): tampoco — `cerrar` haría pop del modal
  //    superior por accidente.
  //  - si es el propio botón ⚙️: tampoco (toggle gestionado en otro sitio).
  document.addEventListener('click', (e) => {
    const menu = document.getElementById('settingsMenu');
    if (!menu?.classList.contains('active')) return;
    const target = e.target as HTMLElement;
    if (menu.contains(target)) return;
    if (target.closest('.modal-overlay')) return;
    if (target.closest('.settings-button')) return;
    safeDispatch('cerrar');
  });
}

run().catch(console.error);
