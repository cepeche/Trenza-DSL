// render.ts — Funciones de render derivadas del estado.
//
// Principio: leen de storage.*, escriben en el DOM. Sin estado propio.
// Se invocan desde efectos Trenza o desde el bucle principal tras dispatch.

import { listTareas, listActividades, listSesiones } from './storage';
import type { TrenzaSystem } from './CronometroPSP_out';
import { Contexto } from './CronometroPSP_out';
import type { Dispatch } from './overlays';

// ─── Helpers de formato ──────────────────────────────────────────────────────

export function formatTime(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  if (h > 0) {
    return `${h}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}

function formatDuration(ms: number): string {
  const total = Math.floor(ms / 1000);
  if (total <= 0) return '0 min';
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  return h > 0 ? `${h}h ${m}min` : `${m}min`;
}

// ─── renderTasksGrid ─────────────────────────────────────────────────────────
// Pinta #gridFrecuentes desde storage. En modo edición, las tarjetas abren
// el modal de edición; en modo normal, inician tarea.

export function renderTasksGrid(dispatch: Dispatch): void {
  const grid = document.getElementById('gridFrecuentes');
  if (!grid) return;

  const tareas = listTareas();
  const isEditing = document.body.classList.contains('editing');

  if (tareas.length === 0) {
    grid.innerHTML =
      '<div style="padding:2rem;text-align:center;color:var(--text-secondary,#888)">' +
      'No hay tareas. Pulsa + para crear una.</div>';
    return;
  }

  grid.innerHTML = '';
  for (const tarea of tareas) {
    const card = document.createElement('button');
    card.className = 'task-card';
    if (isEditing) {
      card.dataset.event = 'abrirEditarTarea';
      card.dataset.payloadTareaId = tarea.id;
    } else {
      card.dataset.event = 'iniciarTarea';
      card.dataset.payloadTareaId = tarea.id;
    }
    card.innerHTML = `
      <div class="task-icon">${tarea.icono}</div>
      <div class="task-name">${tarea.nombre}</div>
    `;
    // Prevent click delegation from also firing on the dispatch inside
    card.addEventListener('click', (e) => {
      e.stopPropagation();
      const event = card.dataset.event!;
      const payload = extractPayloadFromEl(card);
      dispatch(event, payload);
    });
    grid.appendChild(card);
  }
}

// ─── renderActivityButtons ───────────────────────────────────────────────────
// Pinta los botones de actividad en #activityButtons para el modal de
// selección. Se llama cuando ModalSeleccionActividad se activa.

export function renderActivityButtons(tareaId: string, dispatch: Dispatch): void {
  const tareas = listTareas();
  const tarea = tareas.find(t => t.id === tareaId);
  const todasActividades = listActividades();
  const actividades = tarea?.actividadIds.length
    ? todasActividades.filter(a => tarea.actividadIds.includes(a.id))
    : todasActividades;

  const icon = document.getElementById('modalTaskIcon');
  const name = document.getElementById('modalTaskName');
  if (icon) icon.textContent = tarea?.icono ?? '📋';
  if (name) name.textContent = tarea?.nombre ?? 'Tarea';

  const container = document.getElementById('activityButtons');
  if (!container) return;
  container.innerHTML = '';

  for (const act of actividades) {
    const btn = document.createElement('button');
    btn.className = 'activity-btn';
    btn.textContent = act.nombre;
    btn.style.setProperty('--act-color', act.color);
    btn.style.background = act.color;
    btn.style.color = '#fff';
    btn.style.border = 'none';
    btn.style.borderRadius = '0.5rem';
    btn.style.padding = '0.75rem 1.5rem';
    btn.style.cursor = 'pointer';
    btn.style.fontWeight = '600';
    btn.style.width = '100%';
    btn.style.marginBottom = '0.5rem';
    btn.addEventListener('click', (e) => {
      e.stopPropagation();
      dispatch('elegirActividad', { actividadId: act.id });
    });
    container.appendChild(btn);
  }
}

// ─── renderHistorial ─────────────────────────────────────────────────────────
// Pinta #historialContenido. Lee 7 o 30 días según el estado del intérprete.

export function renderHistorial(system: TrenzaSystem): void {
  const contenido = document.getElementById('historialContenido');
  if (!contenido) return;

  const states = system.concurrent_states;
  const dias = states.has(Contexto.Historial30Dias) ? 30 : 7;
  const sesiones = listSesiones(dias);

  // Actualizar botones de periodo
  document.querySelectorAll('.historial-btn-periodo').forEach(btn => {
    const el = btn as HTMLElement;
    const btnDias = parseInt(el.dataset.dias ?? '0', 10);
    el.classList.toggle('active', btnDias === dias);
  });

  if (sesiones.length === 0) {
    contenido.innerHTML =
      '<div style="padding:1.5rem;text-align:center;color:var(--text-secondary,#888)">' +
      `Sin sesiones en los últimos ${dias} días.</div>`;
    return;
  }

  // Agrupar por día
  const byDay = new Map<string, typeof sesiones>();
  for (const s of sesiones) {
    const key = new Date(s.inicio).toLocaleDateString('es-ES', {
      weekday: 'short', day: 'numeric', month: 'short',
    });
    if (!byDay.has(key)) byDay.set(key, []);
    byDay.get(key)!.push(s);
  }

  let html = '';
  for (const [day, daySesiones] of byDay) {
    const totalMs = daySesiones.reduce((sum, s) => sum + (s.fin - s.inicio), 0);
    html += `<div class="historial-dia-header" style="font-weight:700;margin-top:1rem;padding:0.25rem 0;border-bottom:1px solid var(--border,#334155)">`;
    html += `${day} <span style="float:right">${formatDuration(totalMs)}</span></div>`;
    for (const s of daySesiones) {
      const dur = formatDuration(s.fin - s.inicio);
      const hora = new Date(s.inicio).toLocaleTimeString('es-ES', {
        hour: '2-digit', minute: '2-digit',
      });
      html += `<div class="historial-sesion-row" style="padding:0.4rem 0;font-size:0.875rem">`;
      html += `${hora} · ${dur}`;
      if (s.comentario) html += ` · <em>${s.comentario}</em>`;
      html += '</div>';
    }
  }
  contenido.innerHTML = html;
}

// ─── renderTotalToday ────────────────────────────────────────────────────────
// Suma duración de las sesiones del día actual y escribe en #totalToday.

export function renderTotalToday(): void {
  const el = document.getElementById('totalToday');
  if (!el) return;

  const startOfDay = new Date();
  startOfDay.setHours(0, 0, 0, 0);
  const sesiones = listSesiones(1).filter(s => s.inicio >= startOfDay.getTime());
  const totalMs = sesiones.reduce((sum, s) => sum + (s.fin - s.inicio), 0);
  el.textContent = `Hoy: ${formatDuration(totalMs)}`;
}

// ─── renderActiveTimer ───────────────────────────────────────────────────────
// Actualiza el header con nombre de tarea y tiempo transcurrido.

export function renderActiveTimer(taskName: string, seconds: number): void {
  const nameEl = document.getElementById('activeTaskName');
  const timerEl = document.getElementById('timerDisplay');
  if (nameEl) nameEl.textContent = taskName || 'Ninguna tarea activa';
  if (timerEl) timerEl.textContent = seconds > 0 ? formatTime(seconds) : '--:--';
}

// ─── renderResetActivities ───────────────────────────────────────────────────
// Pinta la lista de actividades en Fase 2 del reset (selección a conservar).

export function renderResetActivities(): void {
  const list = document.getElementById('resetActivitiesList');
  if (!list) return;
  const actividades = listActividades();
  list.innerHTML = '';
  for (const act of actividades) {
    const label = document.createElement('label');
    label.style.display = 'flex';
    label.style.alignItems = 'center';
    label.style.gap = '0.5rem';
    label.style.padding = '0.5rem 0';
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.checked = true;
    checkbox.dataset.actividadId = act.id;
    const span = document.createElement('span');
    span.textContent = act.nombre;
    span.style.color = act.color;
    label.appendChild(checkbox);
    label.appendChild(span);
    list.appendChild(label);
  }
}

// ─── Helpers internos ────────────────────────────────────────────────────────

function extractPayloadFromEl(el: HTMLElement): Record<string, string> {
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
