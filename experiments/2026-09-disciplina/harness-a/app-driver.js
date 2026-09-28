// Arranca el frontend original de CronometroPSP (o una variante modificada
// por un agente) bajo jsdom, con una API simulada y datos fijos, y expone
// acciones abstractas (tap sobre elementos) y una instantánea observable.
'use strict';
const fs = require('fs');
const path = require('path');
const { JSDOM, VirtualConsole } = require('jsdom');

const FIXTURES = {
  actividades: [
    { id: 'act1', nombre: 'Trabajo', color: '#667eea', permanente: true },
    { id: 'act2', nombre: 'Casa', color: '#43e97b', permanente: false },
  ],
  tiposTarea: [
    { id: 'tt1', nombre: 'Correo', icono: 'mail', usos_7d: 5, actividades_permitidas: ['act1'] },
    { id: 'tt2', nombre: 'Leer', icono: 'book', usos_7d: 3, actividades_permitidas: ['act1', 'act2'] },
  ],
};

async function boot(frontendDir) {
  const html = fs.readFileSync(path.join(frontendDir, 'index.html'), 'utf8')
    .replace(/<script[^>]*src=[^>]*><\/script>/g, '');
  const calls = [];
  const errors = [];
  const vc = new VirtualConsole();
  vc.on('jsdomError', e => errors.push(String(e && e.message || e)));
  vc.on('error', (...a) => errors.push(a.map(String).join(' ')));
  const dom = new JSDOM(html, { url: 'http://localhost/', runScripts: 'dangerously', pretendToBeVisual: true, virtualConsole: vc });
  const w = dom.window;
  w.fetch = async (url, opts = {}) => {
    const method = (opts.method || 'GET').toUpperCase();
    const u = String(url);
    calls.push({ method, url: u.replace(/^https?:\/\/[^/]+/, ''), body: opts.body || null });
    let data = null;
    if (u.includes('material-icons.json')) data = ['mail', 'book'];
    else if (method === 'GET' && u.includes('/api/actividades')) data = FIXTURES.actividades;
    else if (method === 'GET' && u.includes('/api/tipos-tarea')) data = FIXTURES.tiposTarea;
    else if (method === 'GET' && u.includes('/api/sesiones') && u.includes('activa')) data = null;
    else if (method === 'GET' && u.includes('/api/sesiones')) data = [];
    else data = { success: true, data: { inicio: 0, server_time: 0 } };
    const payload = JSON.stringify(data);
    return { ok: true, status: 200, json: async () => JSON.parse(payload), text: async () => payload };
  };
  w.alert = () => {}; w.confirm = () => true;
  // Un único script (como en el navegador, los `const` de nivel superior de
  // api-client.js deben ser visibles desde app.js).
  const src = ['js/api-client.js', 'js/app.js']
    .map(f => fs.readFileSync(path.join(frontendDir, f), 'utf8')).join('\n;\n');
  try { w.eval(src); } catch (e) { errors.push('eval: ' + e.message); }
  // jsdom dispara DOMContentLoaded de forma asíncrona tras el parseo.
  await new Promise(r => setTimeout(r, 50));
  for (let i = 0; i < 10; i++) await new Promise(r => setImmediate(r));
  return { dom, w, calls, errors };
}

module.exports = { boot, FIXTURES };

if (require.main === module) {
  (async () => {
    const { w, calls, errors } = await boot(process.argv[2]);
    const d = w.document;
    console.log('errors', errors);
    console.log('tabs', [...d.querySelectorAll('.tab')].map(t => t.dataset.tab + (t.classList.contains('active') ? '*' : '')));
    console.log('cards', [...d.querySelectorAll('.task-card')].map(c => (c.closest('[id]') || {}).id + ':' + (c.dataset.tareaId || 'tipo')));
    console.log('calls', calls.map(c => c.method + ' ' + c.url));
    w.close(); process.exit(0);
  })();
}
