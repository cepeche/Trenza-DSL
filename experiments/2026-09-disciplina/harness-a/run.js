#!/usr/bin/env node
// Arnés A: ejecuta escenarios de taps sobre el frontend JS bajo jsdom.
// Uso: node run.js <frontendDir> <escenarios.json>  → JSON en stdout
// Para cada escenario arranca la app desde cero, aplica los taps y devuelve
// la observación tras el último: {mode, overlays, tab, mutations, missing, errors}
'use strict';
const fs = require('fs');
const { boot } = require('./app-driver');

const SELECTORS = {
  tarjeta_tipo: '#gridFrecuentes .task-card',
  tarjeta_tarea: '#grid-act1 .task-card[data-tarea-id="tt1_act1"]',
  pestana_actividad: '.tab[data-tab="act2"]',
  pestana_frecuentes: '.tab[data-tab="frecuentes"]',
  boton_edicion: '#editModeButton',
  boton_nuevo: '.fab-button',
  boton_configuracion: '.settings-button',
  boton_pausa: '#pauseButton',
  boton_cerrar: '#btnReorderClose',
};

const settle = async () => {
  for (let i = 0; i < 5; i++) {
    await new Promise(r => setTimeout(r, 0));
    await new Promise(r => setImmediate(r));
  }
};

function observe(w, calls, mutBefore, missing, errors) {
  const d = w.document;
  const b = d.body.classList;
  const mode = b.contains('pause-mode') ? 'pausa' : b.contains('edit-mode') ? 'edicion' : 'normal';
  const overlays = [...d.querySelectorAll('.modal-overlay.active')].map(e => e.id).sort();
  if (d.getElementById('settingsMenu')?.classList.contains('active')) overlays.push('settingsMenu');
  const tabEl = d.querySelector('.tab.active');
  return {
    mode, overlays, tab: tabEl ? tabEl.dataset.tab : null,
    mutations: calls.filter(c => c.method !== 'GET').length - mutBefore,
    missing, errors: errors.length,
  };
}

(async () => {
  const [dir, scenFile] = process.argv.slice(2);
  const scenarios = JSON.parse(fs.readFileSync(scenFile, 'utf8'));
  const out = [];
  for (const sc of scenarios) {
    const { w, calls, errors } = await boot(dir);
    await settle();
    let last = null;
    for (const role of sc) {
      const mutBefore = calls.filter(c => c.method !== 'GET').length;
      const el = w.document.querySelector(SELECTORS[role] || '#__none__');
      if (el) { el.click(); await settle(); }
      last = observe(w, calls, mutBefore, !el, errors);
    }
    out.push(last);
    w.close();
  }
  process.stdout.write(JSON.stringify(out) + '\n');
  process.exit(0);
})().catch(e => { process.stdout.write(JSON.stringify({ error: 'harness', detail: String(e) }) + '\n'); process.exit(0); });
