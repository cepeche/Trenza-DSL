import { describe, it, expect, beforeEach } from 'vitest';
import { 
  listTareas, 
  createTarea, 
  listActividades, 
  appendSesion, 
  listSesiones, 
  clearAll 
} from './storage';

describe('storage adapter', () => {
  beforeEach(() => {
    clearAll();
  });

  describe('listTareas', () => {
    it('returns empty array when storage is empty', () => {
      expect(listTareas()).toEqual([]);
    });

    it('throws error if storage is corrupted', () => {
      localStorage.setItem('cronometro-psp:v1:tareas', 'invalid-json');
      expect(() => listTareas()).toThrow('storage corrupted: cronometro-psp:v1:tareas');
    });
  });

  describe('createTarea', () => {
    it('creates a task and persists it', () => {
      const input = { nombre: 'Test', icono: '🚀', actividadIds: ['1'] };
      const created = createTarea(input);
      
      expect(created.id).toBeDefined();
      expect(created.nombre).toBe('Test');
      expect(created.creadaEn).toBeLessThanOrEqual(Date.now());
      
      const list = listTareas();
      expect(list).toHaveLength(1);
      expect(list[0]).toEqual(created);
    });
  });

  describe('listActividades', () => {
    it('seeds 3 default activities on first call', () => {
      const list = listActividades();
      expect(list).toHaveLength(3);
      expect(list.map(a => a.nombre)).toContain('Trabajo');
      expect(list.map(a => a.nombre)).toContain('Estudio');
      expect(list.map(a => a.nombre)).toContain('Personal');
    });

    it('does not re-seed on subsequent calls', () => {
      const first = listActividades();
      const second = listActividades();
      expect(first).toEqual(second);
      expect(first).toHaveLength(3);
    });
  });

  describe('appendSesion', () => {
    it('appends a session and retrieves it', () => {
      const input = { 
        tareaId: 't1', 
        actividadId: 'a1', 
        inicio: Date.now() - 1000, 
        fin: Date.now(), 
        comentario: 'Test comment' 
      };
      const created = appendSesion(input);
      expect(created.id).toBeDefined();
      
      const sesiones = listSesiones(1);
      expect(sesiones).toHaveLength(1);
      expect(sesiones[0]).toEqual(created);
    });
  });

  describe('listSesiones', () => {
    it('filters sessions outside the date range', () => {
      const now = Date.now();
      appendSesion({ tareaId: 't1', actividadId: 'a1', inicio: now, fin: now + 1, comentario: 'recent' });
      appendSesion({ tareaId: 't2', actividadId: 'a1', inicio: now - 10 * 86_400_000, fin: now, comentario: 'old' });
      
      expect(listSesiones(7)).toHaveLength(1);
      expect(listSesiones(14)).toHaveLength(2);
    });

    it('sorts sessions by start date descending', () => {
      const now = Date.now();
      appendSesion({ tareaId: 't1', actividadId: 'a1', inicio: now - 2000, fin: now, comentario: 'older' });
      appendSesion({ tareaId: 't2', actividadId: 'a1', inicio: now - 1000, fin: now, comentario: 'newer' });
      
      const list = listSesiones(1);
      expect(list[0].comentario).toBe('newer');
      expect(list[1].comentario).toBe('older');
    });

    it('returns empty array if dias <= 0', () => {
      appendSesion({ tareaId: 't1', actividadId: 'a1', inicio: Date.now(), fin: Date.now() + 1, comentario: 'x' });
      expect(listSesiones(0)).toEqual([]);
    });
  });

  describe('clearAll', () => {
    it('clears only the app-specific keys', () => {
      localStorage.setItem('otra-app:cosa', 'valor');
      createTarea({ nombre: 'T', icono: 'x', actividadIds: [] });
      
      clearAll();
      
      expect(listTareas()).toHaveLength(0);
      expect(localStorage.getItem('otra-app:cosa')).toBe('valor');
    });
  });
});
