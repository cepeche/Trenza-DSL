import { Tarea, Actividad, Sesion } from './storage.types';

const PREFIX = 'cronometro-psp:v1:';
const KEYS = {
  TAREAS: `${PREFIX}tareas`,
  ACTIVIDADES: `${PREFIX}actividades`,
  SESIONES: `${PREFIX}sesiones`,
};

function read<T>(key: string): T[] {
  const data = localStorage.getItem(key);
  if (!data) return [];
  try {
    return JSON.parse(data);
  } catch (e) {
    throw new Error(`storage corrupted: ${key}`);
  }
}

function write<T>(key: string, data: T[]): void {
  localStorage.setItem(key, JSON.stringify(data));
}

export function listTareas(): Tarea[] {
  return read<Tarea>(KEYS.TAREAS);
}

export function createTarea(input: { nombre: string; icono: string; actividadIds: string[] }): Tarea {
  const tareas = listTareas();
  const nuevaTarea: Tarea = {
    ...input,
    id: crypto.randomUUID(),
    creadaEn: Date.now(),
  };
  tareas.push(nuevaTarea);
  write(KEYS.TAREAS, tareas);
  return nuevaTarea;
}

export function listActividades(): Actividad[] {
  let actividades = read<Actividad>(KEYS.ACTIVIDADES);
  if (actividades.length === 0 && !localStorage.getItem(KEYS.ACTIVIDADES)) {
    // Seed inicial
    actividades = [
      { id: crypto.randomUUID(), nombre: 'Trabajo', color: '#0284c7' },
      { id: crypto.randomUUID(), nombre: 'Estudio', color: '#16a34a' },
      { id: crypto.randomUUID(), nombre: 'Personal', color: '#a855f7' },
    ];
    write(KEYS.ACTIVIDADES, actividades);
  }
  return actividades;
}

export function appendSesion(input: { tareaId: string; actividadId: string; inicio: number; fin: number; comentario: string | null }): Sesion {
  const sesiones = read<Sesion>(KEYS.SESIONES);
  const nuevaSesion: Sesion = {
    ...input,
    id: crypto.randomUUID(),
  };
  sesiones.push(nuevaSesion);
  write(KEYS.SESIONES, sesiones);
  return nuevaSesion;
}

export function listSesiones(dias: number): Sesion[] {
  if (dias <= 0) return [];
  const sesiones = read<Sesion>(KEYS.SESIONES);
  const limite = Date.now() - dias * 86_400_000;
  
  return sesiones
    .filter(s => s.inicio >= limite)
    .sort((a, b) => b.inicio - a.inicio);
}

export function clearAll(): void {
  Object.values(KEYS).forEach(key => {
    localStorage.removeItem(key);
  });
}
