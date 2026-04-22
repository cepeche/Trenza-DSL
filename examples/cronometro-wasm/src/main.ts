import init, { InterpreterWasm } from './wasm/trenza_core.js';
import { TrenzaSystem } from './CronometroPSP_out';
import cronometroDsl from './cronometro_full.trz?raw';
import {
  makeDispatchWithSync,
  makeOverlayEffectStubs,
  syncOverlayVisibility,
  wireGlobalCloseHandlers,
} from './overlays';

const log = (msg: string) => {
  const logEl = document.getElementById('log');
  if (logEl) {
    const entry = document.createElement('div');
    entry.className = 'log-entry';
    entry.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
    logEl.prepend(entry);
  }
};

const updateUI = (state: string) => {
  const stateEl = document.getElementById('state');
  if (stateEl) stateEl.textContent = state;
};

async function run() {
  log("Iniciando motor WASM...");
  await init();

  let seconds = 0;
  let timerInterval: number | undefined;

  const formatTime = (totalSeconds: number) => {
    const hrs = Math.floor(totalSeconds / 3600).toString().padStart(2, '0');
    const mins = Math.floor((totalSeconds % 3600) / 60).toString().padStart(2, '0');
    const secs = (totalSeconds % 60).toString().padStart(2, '0');
    return `${hrs}:${mins}:${secs}`;
  };

  const startTimer = () => {
    if (timerInterval) return;
    timerInterval = setInterval(() => {
      seconds++;
      const display = document.getElementById('display');
      if (display) display.textContent = formatTime(seconds);
    }, 1000);
  };

  const stopTimer = () => {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = undefined;
    }
  };

  // Efectos: los `abrir*` y `cerrar` son no-ops (la visibilidad la proyecta
  // el estado desde overlays.ts). El resto son acciones de negocio locales
  // al demo — Fase 3 los sustituira por llamadas al storage adapter.
  const effects = {
    ...makeOverlayEffectStubs(),
    iniciar_sesion: (tareaId: string) => {
      log(`Efecto de Negocio: Iniciar Sesion para Tarea ${tareaId}`);
      startTimer();
    },
    parar_sesion: () => {
      log("Efecto de Negocio: Detener Sesion");
      stopTimer();
    },
    confirmarInicio: () => log("Validacion: Inicio confirmado por usuario"),
    reset_datos: () => {
      log("Efecto de Negocio: Reset de todos los datos");
      seconds = 0;
      const display = document.getElementById('display');
      if (display) display.textContent = "00:00:00";
    },
  } as any;

  log("Cargando especificacion Cronometro-PSP...");
  const interpreter = new InterpreterWasm(cronometroDsl);
  const system = new TrenzaSystem(interpreter, effects);
  const dispatch = makeDispatchWithSync(system);

  wireGlobalCloseHandlers(dispatch);
  syncOverlayVisibility(system);

  log("Sistema Trenza Ready.");
  updateUI(system.current_state);

  // Los cuatro botones del demo actual. Cuando Brief A porte el HTML real,
  // estos handlers pueden permanecer (los ids siguen existiendo como puntos
  // de entrada minimos) o sustituirse por delegacion sobre data-event.
  const wire = (id: string, event: string, payload?: unknown) => {
    document.getElementById(id)?.addEventListener('click', () => {
      log(`Evento: '${event}' disparado`);
      dispatch(event, payload);
      updateUI(system.current_state);
    });
  };

  wire('btn-iniciar', 'iniciarTarea', { tareaId: 'TASK-001' });
  wire('btn-parar',   'terminarSesion');
  wire('btn-config',  'abrirMenuConfiguracion');
  wire('btn-reset',   'abrirReset');
}

run().catch(e => {
  console.error(e);
  log(`Error: ${e.message}`);
});
