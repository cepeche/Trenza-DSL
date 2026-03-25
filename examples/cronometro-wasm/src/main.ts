import init, { InterpreterWasm } from './wasm/trenza_core.js';
import { TrenzaSystem, Contexto } from './CronometroPSP_out';

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

  // 1. Implementación de Efectos (Minimal para el demo)
  const effects = {
    iniciar_sesion: (tareaId: string) => log(`Efecto de Negocio: Iniciar Sesión para Tarea ${tareaId}`),
    parar_sesion: () => log("Efecto de Negocio: Detener Sesión"),
    abrirMenuConfiguracion: () => log("UI: Abriendo Menú Configuración"),
    abrirReset: () => log("UI: Abriendo Modal Reset"),
    confirmarInicio: () => log("Validación: Inicio confirmado por usuario"),
    // El resto de los 183 efectos pueden quedar como no-ops para este demo
  } as any;

  // 2. Instanciar Sistema
  const interpreter = new InterpreterWasm();
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
    log("Evento: 'cerrar' disparado");
    system.dispatch('cerrar');
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
