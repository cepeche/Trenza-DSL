import init, { InterpreterWasm } from './wasm/trenza_core.js';
import { TrenzaSystem } from './CronometroPSP_out';
import cronometroDsl from './cronometro_full.trz?raw';

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

  // 1. Implementación de Efectos
  const effects = {
    iniciar_sesion: (tareaId: string) => {
      log(`Efecto de Negocio: Iniciar Sesión para Tarea ${tareaId}`);
      startTimer();
    },
    parar_sesion: () => {
      log("Efecto de Negocio: Detener Sesión");
      stopTimer();
    },
    abrirMenuConfiguracion: () => log("UI: Abriendo Menú Configuración"),
    abrirReset: () => {
      log("UI: Abriendo Modal Reset");
      stopTimer();
    },
    confirmarInicio: () => log("Validación: Inicio confirmado por usuario"),
    reset_datos: () => {
      log("Efecto de Negocio: Reset de todos los datos");
      seconds = 0;
      const display = document.getElementById('display');
      if (display) display.textContent = "00:00:00";
    }
  } as any;

  // 2. Instanciar Sistema (Pasando el DSL consolidado)
  log("Cargando especificación Cronómetro-PSP...");
  const interpreter = new InterpreterWasm(cronometroDsl);
  const system = new TrenzaSystem(interpreter, effects);

  log("Sistema Trenza Ready.");
  updateUI(system.current_state);

  // 3. Vincular Eventos del DOM a Despachos de Trenza
  document.getElementById('btn-iniciar')?.addEventListener('click', () => {
    log("Evento: 'iniciarTarea' disparado");
    system.dispatch('iniciarTarea', { tareaId: 'TASK-001' });
    updateUI(system.current_state);
  });

  document.getElementById('btn-parar')?.addEventListener('click', () => {
    log("Evento: 'terminarSesion' disparado");
    system.dispatch('terminarSesion');
    updateUI(system.current_state);
  });

  document.getElementById('btn-config')?.addEventListener('click', () => {
    log("Evento: 'abrirMenuConfiguracion' disparado");
    system.dispatch('abrirMenuConfiguracion');
    updateUI(system.current_state);
  });

  document.getElementById('btn-reset')?.addEventListener('click', () => {
    log("Evento: 'abrirReset' disparado");
    system.dispatch('abrirReset');
    updateUI(system.current_state);
  });
}

run().catch(e => {
  console.error(e);
  log(`Error: ${e.message}`);
});
