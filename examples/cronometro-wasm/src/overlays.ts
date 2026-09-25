// Fase 2 — capa de efectos sobre overlays.
//
// Trenza declara 10 overlays en cronometro_full.trz. La visibilidad de cada
// modal en el DOM es una proyeccion directa del conjunto de estados activos
// en el interprete: no hay ningun `if` de UI que decida que mostrar.
//
// - OVERLAY_DOM_IDS es el unico punto donde Trenza y el DOM se acuerdan.
// - syncOverlayVisibility() re-proyecta el estado al DOM tras cada dispatch.
// - makeDispatchWithSync() envuelve system.dispatch para que la proyeccion
//   sea automatica.
// - makeOverlayEffectStubs() cubre los efectos `abrir*` / `cerrar` declarados
//   en el .trz como no-ops auditables: la apertura sucede por transicion de
//   estado, no por manipulacion directa del DOM.
// - wireGlobalCloseHandlers() conecta Esc y click-en-backdrop al dispatcher.

import type { Effects } from './CronometroPSP_out';
import type { TrenzaSystem } from './snapshot-bridge';

export const OVERLAY_DOM_IDS: Readonly<Record<string, string>> = Object.freeze({
  MenuConfiguracion:       'settingsMenu',
  ModalSeleccionActividad: 'activityModal',
  ModalCrearTarea:         'createTaskModal',
  ModalCrearActividad:     'createActivityModal',
  ModalComentario:         'commentModal',
  ModalAcercaDe:           'aboutModal',
  ModalReset:              'resetModal',
  ModalEditarTarea:        'editTaskModal',
  ModalEditarActividad:    'editActivityModal',
  ModalHistorial:          'historialModal',
  // Sub-contexts: when the runtime pushes a sub-context onto overlay_stack,
  // its parent overlay is the one that should remain visible. The shim's
  // `parent_overlay_of` resolves that at compile-time, but the overlay_stack
  // surface still contains the sub-context name, so we alias it here.
  Historial7Dias:          'historialModal',
  Historial30Dias:         'historialModal',
  ResetFase1:              'resetModal',
  ResetFase2:              'resetModal',
  ResetFase3:              'resetModal',
});

export function syncOverlayVisibility(system: TrenzaSystem): void {
  const active = new Set<string>();
  active.add(system.current_state);
  // overlay_stack contains every active overlay (including sub-contexts).
  for (const s of system.overlay_stack) active.add(s);
  for (const s of system.concurrent_states) active.add(s);

  // Project: an overlay is visible iff itself or one of its sub-contexts is
  // active. OVERLAY_DOM_IDS aliases sub-contexts to the same DOM id, so a
  // single OR over the table is enough.
  const visibleDomIds = new Set<string>();
  for (const [overlay, domId] of Object.entries(OVERLAY_DOM_IDS)) {
    if (active.has(overlay)) visibleDomIds.add(domId);
  }
  // Apply: pass over each unique DOM id once, toggling .active accordingly.
  const allDomIds = new Set(Object.values(OVERLAY_DOM_IDS));
  for (const domId of allDomIds) {
    const el = document.getElementById(domId);
    if (!el) continue;
    el.classList.toggle('active', visibleDomIds.has(domId));
  }

  document.body.classList.toggle('editing', active.has('ModoEdicion'));
}

export type Dispatch = (event: string, payload?: unknown) => void;

export function makeDispatchWithSync(system: TrenzaSystem): Dispatch {
  return (event, payload = {}) => {
    system.dispatch(event, payload);
    syncOverlayVisibility(system);
  };
}

export function wireGlobalCloseHandlers(dispatch: Dispatch): void {
  document.addEventListener('keydown', (e) => {
    if (e.key !== 'Escape') return;
    dispatch('cancelar');
    dispatch('cerrar');
  });

  document.addEventListener('click', (e) => {
    const target = e.target as HTMLElement | null;
    if (!target) return;
    if (target.classList?.contains('modal-overlay')) {
      dispatch('cancelar');
      dispatch('cerrar');
    }
  });
}

// Los efectos `abrir*` y `cerrar` son declarados en el .trz pero la
// visibilidad la resuelve la proyeccion de estado. Los stubs aqui son
// no-ops auditables: el perfil --profile=pre puede inyectar telemetria,
// y el compilador los usa para verificar la exhaustividad del tejido.
export function makeOverlayEffectStubs(): Pick<
  Effects,
  | 'abrirAcercaDe'
  | 'abrirCrearActividad'
  | 'abrirCrearTarea'
  | 'abrirEditarActividad'
  | 'abrirEditarTarea'
  | 'abrirHistorial'
  | 'abrirMenuConfiguracion'
  | 'abrirReset'
  | 'cerrar'
  | 'cancelar'
> {
  return {
    abrirAcercaDe:          () => {},
    abrirCrearActividad:    () => {},
    abrirCrearTarea:        () => {},
    abrirEditarActividad:   (_id) => {},
    abrirEditarTarea:       (_id) => {},
    abrirHistorial:         () => {},
    abrirMenuConfiguracion: () => {},
    abrirReset:             () => {},
    cerrar:                 () => {},
    cancelar:               () => {},
  };
}
