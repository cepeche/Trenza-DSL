export interface Tarea {
  id: string;          // uuid v4 generado por createTarea
  nombre: string;
  icono: string;       // emoji
  actividadIds: string[];
  creadaEn: number;    // ms epoch
}

export interface Actividad {
  id: string;          // uuid v4 generado en seed o por crearActividad
  nombre: string;
  color: string;       // hex
}

export interface Sesion {
  id: string;          // uuid v4 generado por appendSesion
  tareaId: string;
  actividadId: string;
  inicio: number;      // ms epoch
  fin: number;         // ms epoch
  comentario: string | null;
}
