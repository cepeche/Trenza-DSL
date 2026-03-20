/**
 * app.functions.test.js — Tests unitarios para funciones puras de app.js
 *
 * Se ejecuta a través de tests/js/runner.js (no directamente).
 * Las funciones bajo test están disponibles como globales porque runner.js
 * ejecutó app.js con vm.runInThisContext.
 *
 * Funciones cubiertas:
 *   formatearDuracion()        — conversión segundos → "H:MM"
 *   getTareasFrecuentes()      — top 8 por usos_7d + recientes de hoy
 *   getTiempoHoyTipoTarea()    — suma sesiones cerradas + activa del tipo
 *   getTiempoHoyTarea()        — suma sesiones cerradas + activa de la tarea
 */

'use strict';

// ─── Helper: resetear AppState a estado vacío antes de cada grupo ─────────────

function resetState(overrides = {}) {
    AppState.actividades      = {};
    AppState.tiposTarea       = [];
    AppState.tareas           = [];
    AppState.sesionesHoy      = [];
    AppState.sesionActiva     = null;
    AppState.notasActivas     = null;
    AppState.tareaIdPendiente = null;
    Object.assign(AppState, overrides);
}

// ─── formatearDuracion ────────────────────────────────────────────────────────

test('formatearDuracion: 0 segundos → "0:00"', () => {
    expect(formatearDuracion(0)).toBe('0:00');
});

test('formatearDuracion: 59 segundos → "0:00" (trunca, no redondea)', () => {
    expect(formatearDuracion(59)).toBe('0:00');
});

test('formatearDuracion: 60 segundos → "0:01"', () => {
    expect(formatearDuracion(60)).toBe('0:01');
});

test('formatearDuracion: 90 segundos → "0:01"', () => {
    expect(formatearDuracion(90)).toBe('0:01');
});

test('formatearDuracion: 3600 segundos → "1:00"', () => {
    expect(formatearDuracion(3600)).toBe('1:00');
});

test('formatearDuracion: 3661 segundos → "1:01"', () => {
    expect(formatearDuracion(3661)).toBe('1:01');
});

test('formatearDuracion: 7380 segundos → "2:03"', () => {
    expect(formatearDuracion(7380)).toBe('2:03');
});

test('formatearDuracion: minutos < 10 se pad con cero ("1:05")', () => {
    expect(formatearDuracion(3900)).toBe('1:05');
});

test('formatearDuracion: minutos ≥ 10 sin cero adicional ("1:10")', () => {
    expect(formatearDuracion(4200)).toBe('1:10');
});

test('formatearDuracion: valores grandes (10 horas) → "10:00"', () => {
    expect(formatearDuracion(36000)).toBe('10:00');
});

// ─── getTareasFrecuentes ──────────────────────────────────────────────────────

test('getTareasFrecuentes: lista vacía si todos usos_7d = 0', () => {
    resetState({
        tiposTarea: [
            { id: 't1', usos_7d: 0 },
            { id: 't2', usos_7d: 0 },
        ],
    });
    expect(getTareasFrecuentes()).toHaveLength(0);
});

test('getTareasFrecuentes: solo incluye tipos con usos_7d > 0', () => {
    resetState({
        tiposTarea: [
            { id: 't1', usos_7d: 5 },
            { id: 't2', usos_7d: 0 },
            { id: 't3', usos_7d: 3 },
        ],
    });
    const result = getTareasFrecuentes();
    expect(result).toHaveLength(2);
    expect(result.map(t => t.id)).toContain('t1');
    expect(result.map(t => t.id)).toContain('t3');
    expect(result.map(t => t.id)).not.toContain('t2');
});

test('getTareasFrecuentes: máximo 8 resultados del top', () => {
    resetState({
        tiposTarea: Array.from({ length: 12 }, (_, i) => ({
            id: `t${i}`,
            usos_7d: i + 1,
        })),
    });
    expect(getTareasFrecuentes()).toHaveLength(8);
});

test('getTareasFrecuentes: ordenados por usos_7d DESC', () => {
    resetState({
        tiposTarea: [
            { id: 'tA', usos_7d: 2 },
            { id: 'tB', usos_7d: 8 },
            { id: 'tC', usos_7d: 5 },
        ],
    });
    const result = getTareasFrecuentes();
    expect(result[0].id).toBe('tB');
    expect(result[1].id).toBe('tC');
    expect(result[2].id).toBe('tA');
});

test('getTareasFrecuentes: tipo reciente de hoy se añade si no está en el top8', () => {
    // 8 tipos en el top con usos altos + uno con usos_7d=0 pero reciente hoy
    const top8 = Array.from({ length: 8 }, (_, i) => ({ id: `top${i}`, usos_7d: 10 - i }));
    resetState({
        tiposTarea: [...top8, { id: 'reciente', usos_7d: 0 }],
        sesionesHoy: [
            { tipo_tarea_id: 'reciente' },
            { tipo_tarea_id: 'reciente' },
            { tipo_tarea_id: 'reciente' },
        ],
    });
    const result = getTareasFrecuentes();
    expect(result).toHaveLength(9); // 8 del top + 1 reciente
    expect(result.map(t => t.id)).toContain('reciente');
});

test('getTareasFrecuentes: tipo reciente no se duplica si ya está en el top8', () => {
    resetState({
        tiposTarea: [{ id: 'ya_en_top', usos_7d: 10 }],
        sesionesHoy: [
            { tipo_tarea_id: 'ya_en_top' },
            { tipo_tarea_id: 'ya_en_top' },
        ],
    });
    const result = getTareasFrecuentes();
    const ocurrencias = result.filter(t => t.id === 'ya_en_top');
    expect(ocurrencias).toHaveLength(1);
});

test('getTareasFrecuentes: recientes solo considera las últimas 3 sesiones', () => {
    // 8 tipos en el top. Un tipo muy reciente aparece solo en sesiones[0..1]
    // (más de 3 posiciones atrás) y otro en sesiones[0]
    const top8 = Array.from({ length: 8 }, (_, i) => ({ id: `top${i}`, usos_7d: 10 }));
    resetState({
        tiposTarea: [
            ...top8,
            { id: 'reciente_cercano',  usos_7d: 0 },
            { id: 'reciente_lejano',   usos_7d: 0 },
        ],
        // sesionesHoy: las 3 últimas tienen 'reciente_cercano'; 'reciente_lejano' está antes
        sesionesHoy: [
            { tipo_tarea_id: 'reciente_lejano' },   // sesión más vieja (índice 0)
            { tipo_tarea_id: 'reciente_cercano' },  // índice 1 → slice(-3) lo incluye
            { tipo_tarea_id: 'reciente_cercano' },
            { tipo_tarea_id: 'reciente_cercano' },
        ],
    });
    const result = getTareasFrecuentes();
    const ids = result.map(t => t.id);
    expect(ids).toContain('reciente_cercano');
    expect(ids).not.toContain('reciente_lejano');
});

// ─── getTiempoHoyTipoTarea ────────────────────────────────────────────────────

test('getTiempoHoyTipoTarea: suma correctamente sesiones cerradas (strings PDO)', () => {
    resetState({
        sesionesHoy: [
            { tipo_tarea_id: 'codificar', duracion: '3600' }, // strings como devuelve PDO
            { tipo_tarea_id: 'codificar', duracion: '1800' },
            { tipo_tarea_id: 'email',     duracion: '900'  },
        ],
    });
    expect(getTiempoHoyTipoTarea('codificar')).toBe(5400);
});

test('getTiempoHoyTipoTarea: devuelve 0 si no hay sesiones del tipo', () => {
    resetState({
        sesionesHoy: [{ tipo_tarea_id: 'email', duracion: '900' }],
    });
    expect(getTiempoHoyTipoTarea('codificar')).toBe(0);
});

test('getTiempoHoyTipoTarea: ignora sesiones sin duración (sesiones activas, duracion=null)', () => {
    resetState({
        sesionesHoy: [
            { tipo_tarea_id: 'codificar', duracion: null   },
            { tipo_tarea_id: 'codificar', duracion: '1800' },
        ],
        sesionActiva: null,
    });
    expect(getTiempoHoyTipoTarea('codificar')).toBe(1800);
});

test('getTiempoHoyTipoTarea: suma sesión activa si es del mismo tipo', () => {
    const inicioHace120s = new Date(Date.now() - 120_000);
    resetState({
        sesionesHoy: [{ tipo_tarea_id: 'codificar', duracion: '3600' }],
        tareas: [{ id: 'tt_act', tipoTareaId: 'codificar', actividadId: 'act1' }],
        sesionActiva: { tareaId: 'tt_act', inicio: inicioHace120s },
    });
    const resultado = getTiempoHoyTipoTarea('codificar');
    expect(resultado).toBeGreaterThanOrEqual(3720); // 3600 + 120
    expect(resultado).toBeLessThan(3840);           // margen de 2 minutos
});

test('getTiempoHoyTipoTarea: no suma sesión activa de otro tipo', () => {
    resetState({
        sesionesHoy: [{ tipo_tarea_id: 'codificar', duracion: '3600' }],
        tareas: [{ id: 'tt2_act', tipoTareaId: 'email', actividadId: 'act1' }],
        sesionActiva: { tareaId: 'tt2_act', inicio: new Date(Date.now() - 120_000) },
    });
    expect(getTiempoHoyTipoTarea('codificar')).toBe(3600);
});

test('getTiempoHoyTipoTarea: sesión activa solo cuenta desde medianoche si empezó ayer', () => {
    const medianoche = new Date();
    medianoche.setHours(0, 0, 0, 0);
    const inicioAyer = new Date(medianoche.getTime() - 3_600_000); // 1h antes de medianoche

    resetState({
        sesionesHoy: [],
        tareas: [{ id: 'tt_act', tipoTareaId: 'codificar', actividadId: 'act1' }],
        sesionActiva: { tareaId: 'tt_act', inicio: inicioAyer },
    });

    const resultado = getTiempoHoyTipoTarea('codificar');
    const maxEsperado = Math.floor((Date.now() - medianoche.getTime()) / 1000) + 5;
    // Debe estar acotado al tiempo desde medianoche (no contar la hora de ayer)
    expect(resultado).toBeLessThanOrEqual(maxEsperado);
    // Debe ser significativamente menor que si contase desde inicioAyer (sin capar)
    const sinCapar = Math.floor((Date.now() - inicioAyer.getTime()) / 1000);
    expect(resultado).toBeLessThan(sinCapar - 3000);
});

// ─── getTiempoHoyTarea ────────────────────────────────────────────────────────

test('getTiempoHoyTarea: suma sesiones cerradas de esa tarea exacta', () => {
    resetState({
        sesionesHoy: [
            { tarea_id: 'tt1_act1', duracion: '3600' },
            { tarea_id: 'tt1_act1', duracion: '900'  },
            { tarea_id: 'tt2_act2', duracion: '1800' }, // otra tarea
        ],
    });
    expect(getTiempoHoyTarea('tt1_act1')).toBe(4500);
});

test('getTiempoHoyTarea: devuelve 0 si no hay sesiones de esa tarea', () => {
    resetState({
        sesionesHoy: [{ tarea_id: 'tt2_act2', duracion: '1800' }],
    });
    expect(getTiempoHoyTarea('tt1_act1')).toBe(0);
});

test('getTiempoHoyTarea: suma sesión activa si esa tarea está activa', () => {
    const inicioHace120s = new Date(Date.now() - 120_000);
    resetState({
        sesionesHoy: [{ tarea_id: 'tt1_act1', duracion: '3600' }],
        sesionActiva: { tareaId: 'tt1_act1', inicio: inicioHace120s },
    });
    const resultado = getTiempoHoyTarea('tt1_act1');
    expect(resultado).toBeGreaterThanOrEqual(3720);
    expect(resultado).toBeLessThan(3840);
});

test('getTiempoHoyTarea: no suma sesión activa si es otra tarea', () => {
    resetState({
        sesionesHoy: [{ tarea_id: 'tt1_act1', duracion: '3600' }],
        sesionActiva: { tareaId: 'tt2_act2', inicio: new Date(Date.now() - 120_000) },
    });
    expect(getTiempoHoyTarea('tt1_act1')).toBe(3600);
});

test('getTiempoHoyTarea: sesión activa limitada a medianoche si empezó ayer', () => {
    const medianoche = new Date();
    medianoche.setHours(0, 0, 0, 0);
    const inicioAyer = new Date(medianoche.getTime() - 3_600_000);

    resetState({
        sesionesHoy: [],
        sesionActiva: { tareaId: 'tt1_act1', inicio: inicioAyer },
    });

    const resultado = getTiempoHoyTarea('tt1_act1');
    const maxEsperado = Math.floor((Date.now() - medianoche.getTime()) / 1000) + 5;
    // Debe estar acotado al tiempo desde medianoche (no contar la hora de ayer)
    expect(resultado).toBeLessThanOrEqual(maxEsperado);
    // Debe ser significativamente menor que si contase desde inicioAyer (sin capar)
    const sinCapar = Math.floor((Date.now() - inicioAyer.getTime()) / 1000);
    expect(resultado).toBeLessThan(sinCapar - 3000);
});

test('getTiempoHoyTarea: no hay concatenación string (bug PDO "3600"+"900"="3600900")', () => {
    // Regresión: PDO devuelve duracion como string; si se suma con + en lugar de Number()
    // el resultado sería "3600900" no 4500.
    resetState({
        sesionesHoy: [
            { tarea_id: 'tt1_act1', duracion: '3600' },
            { tarea_id: 'tt1_act1', duracion: '900'  },
        ],
    });
    const resultado = getTiempoHoyTarea('tt1_act1');
    expect(resultado).toBe(4500);
    expect(resultado).not.toBe('3600900'); // la cadena que produciría el bug
});
