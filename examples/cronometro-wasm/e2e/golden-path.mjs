// Recorrido de extremo a extremo de la demo en Chromium (Playwright).
// Uso: npm run build && npx vite preview --port 4173 &  node e2e/golden-path.mjs
// Requiere el paquete 'playwright' (npm i -D playwright) y un Chromium;
// Si la versión de Playwright no coincide con el Chromium instalado, pasa
// CHROMIUM_PATH=/ruta/al/chrome (en Claude Code on the web:
// /opt/pw-browsers/chromium-1194/chrome-linux/chrome).
// Sale con código 1 si falla alguna comprobación.
import { chromium } from 'playwright';
const browser = await chromium.launch(process.env.CHROMIUM_PATH ? { executablePath: process.env.CHROMIUM_PATH } : {});
const page = await browser.newPage();
const errors = [];
page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });
page.on('pageerror', e => errors.push('pageerror: ' + e.message));
await page.goto(process.env.URL || 'http://localhost:4173/');
await page.waitForTimeout(1500);
const state = async (label) => {
  const s = await page.evaluate(() => ({
    overlays: [...document.querySelectorAll('.modal-overlay.active')].map(e => e.id),
    settings: document.getElementById('settingsMenu')?.classList.contains('active'),
    body: document.body.className,
    timer: document.getElementById('timerDisplay')?.textContent?.trim(),
    task: document.getElementById('activeTaskName')?.textContent?.trim(),
    cards: document.querySelectorAll('.task-card').length,
    step: ['resetStep1','resetStep2','resetStep3'].find(id => document.getElementById(id)?.style.display === 'block') || null,
  }));
  console.log(label.padEnd(38), JSON.stringify(s));
  return s;
};
const click = async (sel) => { await page.locator(sel).first().click(); await page.waitForTimeout(300); };
const R = {};
await state('0 arranque');
await click('#editModeButton'); R.edicionOnInicio = /edit/.test((await state('0a modo edición')).body);
await click('#editModeButton'); R.edicionOffInicio = !/edit/.test((await state('0b salir edición')).body);
await click('[data-event="abrirCrearTarea"]'); R.crear = (await state('1 + → crear tarea')).overlays.includes('createTaskModal');
await page.fill('#newTaskName', 'Correo'); await click('#btnGuardarTarea'); R.guardar = (await state('2 guardar tarea')).cards > 0;
await click('.task-card');
let s = await state('3 tap tarjeta');
if (s.overlays.includes('activityModal')) { await click('#activityButtons button'); s = await state('3b elegir actividad'); }
const conComentario = s.overlays.includes('commentModal'); // según la spec, tocar una tarea inicia la sesión sin diálogo
if (conComentario) { await click('#commentModal [data-event="confirmarInicio"]'); }
await page.waitForTimeout(1500);
s = await state('4 sesión iniciada'); R.iniciar = s.overlays.length === 0 && s.task !== 'Ninguna tarea activa';
await click('.active-timer'); s = await state('5 parar'); R.parar = s.timer === '--:--' || s.task?.includes('Ninguna');
await click('[data-event="abrirMenuConfiguracion"]'); await click('#settingsMenu [data-event="abrirHistorial"]'); s = await state('6 historial'); R.historial = s.overlays.includes('historialModal');
await click('#historialModal [data-event="cerrar"]'); s = await state('7 cerrar historial'); R.cerrarHist = s.overlays.length === 0;
await click('[data-event="abrirMenuConfiguracion"]'); await click('#settingsMenu [data-event="abrirReset"]'); s = await state('8 reset fase 1'); R.reset1 = s.overlays.includes('resetModal') && s.step === 'resetStep1';
await click('#btnReset'); s = await state('9 reset fase 2'); R.reset2 = s.step === 'resetStep2';
await click('#btnResetCancel'); s = await state('10 cancelar en fase 2'); R.cancelarF2 = !s.overlays.includes('resetModal');
await click('[data-event="abrirMenuConfiguracion"]'); await click('#settingsMenu [data-event="abrirReset"]');
await click('#btnReset'); await click('#btnReset'); s = await state('11 reset fase 3'); R.reset3 = s.step === 'resetStep3';
await page.fill('#resetConfirmInput', 'BORRAR'); await click('#btnReset'); s = await state('12 ejecutar reset'); R.ejecutar = !s.overlays.includes('resetModal');
await click('#editModeButton'); s = await state('13 edición tras usar el menú'); R.edicionTrasMenu = /edit/.test(s.body);
await click('#editModeButton'); s = await state('14 segundo toque');
console.log('\nRESULTADO', JSON.stringify(R));
console.log('ERRORES', JSON.stringify(errors));
if (process.env.SHOT) await page.screenshot({ path: process.env.SHOT });
await browser.close();
process.exit(Object.values(R).every(v => v !== false || false) ? 0 : 1);
