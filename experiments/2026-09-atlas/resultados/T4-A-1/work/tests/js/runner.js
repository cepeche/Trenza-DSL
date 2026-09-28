/**
 * runner.js — Test runner mínimo para Node.js puro (sin npm, sin dependencias)
 *
 * Uso: node tests/js/runner.js
 *
 * Estrategia:
 * 1. Define globales stub (document=null, fetch=null, etc.) que app.js necesita
 *    pero que no existen en Node.
 * 2. Carga app.js con vm.runInThisContext para que las funciones queden en el
 *    scope global (const/let del top-level de un módulo no son globales, pero
 *    vm.runInThisContext sí los hace accesibles).
 * 3. Parchea en memoria la última línea de app.js para desactivar el listener
 *    DOMContentLoaded (document es null → lanzaría TypeError).
 * 4. Expone test() y expect() como globales para que app.functions.test.js
 *    los use sin imports.
 */

'use strict';

const fs   = require('fs');
const vm   = require('vm');
const path = require('path');

// ─── Mini framework de aserciones ────────────────────────────────────────────

const _tests = [];
let _passed = 0;
let _failed = 0;

global.test = function test(name, fn) {
    _tests.push({ name, fn });
};

global.expect = function expect(actual) {
    return {
        toBe(expected) {
            if (actual !== expected)
                throw new Error(`Expected ${_fmt(expected)}, got ${_fmt(actual)}`);
        },
        toEqual(expected) {
            if (JSON.stringify(actual) !== JSON.stringify(expected))
                throw new Error(`Expected ${_fmt(expected)}, got ${_fmt(actual)}`);
        },
        toHaveLength(n) {
            if (!actual || actual.length !== n)
                throw new Error(`Expected length ${n}, got ${actual ? actual.length : 'undefined'}`);
        },
        toBeGreaterThan(n) {
            if (actual <= n) throw new Error(`Expected > ${n}, got ${actual}`);
        },
        toBeGreaterThanOrEqual(n) {
            if (actual < n) throw new Error(`Expected >= ${n}, got ${actual}`);
        },
        toBeLessThan(n) {
            if (actual >= n) throw new Error(`Expected < ${n}, got ${actual}`);
        },
        toBeLessThanOrEqual(n) {
            if (actual > n) throw new Error(`Expected <= ${n}, got ${actual}`);
        },
        toBeNull() {
            if (actual !== null) throw new Error(`Expected null, got ${_fmt(actual)}`);
        },
        toBeUndefined() {
            if (actual !== undefined) throw new Error(`Expected undefined, got ${_fmt(actual)}`);
        },
        toBeTruthy() {
            if (!actual) throw new Error(`Expected truthy, got ${_fmt(actual)}`);
        },
        toBeFalsy() {
            if (actual) throw new Error(`Expected falsy, got ${_fmt(actual)}`);
        },
        toContain(item) {
            if (!Array.isArray(actual) ? !actual.includes(item) : !actual.includes(item))
                throw new Error(`Expected ${_fmt(actual)} to contain ${_fmt(item)}`);
        },
        not: {
            toContain(item) {
                if (actual.includes(item))
                    throw new Error(`Expected ${_fmt(actual)} NOT to contain ${_fmt(item)}`);
            },
            toBe(expected) {
                if (actual === expected)
                    throw new Error(`Expected NOT ${_fmt(expected)}, but got it`);
            },
        },
    };
};

function _fmt(v) {
    return JSON.stringify(v);
}

// ─── Stubs de globales del navegador ─────────────────────────────────────────

global.document    = null;   // app.js comprueba pero no usa en funciones puras
global.fetch       = null;
global.setInterval = () => {};
global.clearInterval = () => {};
global.setTimeout  = (fn, _ms) => { /* no-op en tests */ };
global.alert       = () => {};
global.URL         = { createObjectURL: () => '', revokeObjectURL: () => {} };
global.API_CONFIG  = { baseURL: 'http://localhost' };

// AppState global controlado por los tests
global.AppState = {
    actividades:          {},
    tiposTarea:           [],
    tareas:               [],
    sesionesHoy:          [],
    sesionActiva:         null,
    notasActivas:         null,
    tareaIdPendiente:     null,
    timerInterval:        null,
    pestanaActual:        'frecuentes',
    modoEdicion:          false,
    iconoSeleccionado:    'draft',
    iconoEditSeleccionado:'draft',
    colorSeleccionado:    '#667eea',
    todosIconos:          [],
    coloresDisponibles:   [],
};

// ─── Cargar app.js con vm ─────────────────────────────────────────────────────

const appJsPath = path.resolve(__dirname, '../../frontend/js/app.js');
let   appJsCode = fs.readFileSync(appJsPath, 'utf8');

// Neutralizar DOMContentLoaded (document es null → TypeError)
appJsCode = appJsCode.replace(
    "document.addEventListener('DOMContentLoaded', inicializarApp);",
    "// TEST_ENV: DOMContentLoaded desactivado"
);

// Neutralizar referencias a `api` (APIClient) que no existe en el entorno de test
// Las funciones puras que testeamos no lo usan, pero la declaración de `const api`
// al final de api-client.js podría fallar. Lo manejamos definiendo un stub:
global.api = {};

try {
    vm.runInThisContext(appJsCode, { filename: 'app.js', displayErrors: true });
} catch (err) {
    console.error('Error al cargar app.js:', err.message);
    process.exit(1);
}

// ─── Cargar archivo de tests ──────────────────────────────────────────────────

require('./app.functions.test.js');

// ─── Ejecutar todos los tests ─────────────────────────────────────────────────

async function runAll() {
    console.log('\nMi Cronómetro PSP — JS Unit Tests');
    console.log('===================================\n');

    for (const { name, fn } of _tests) {
        try {
            await fn();
            console.log(`  ✓  ${name}`);
            _passed++;
        } catch (e) {
            console.log(`  ✗  ${name}`);
            console.log(`     → ${e.message}`);
            _failed++;
        }
    }

    console.log(`\n${_passed + _failed} tests: ${_passed} passed, ${_failed} failed\n`);

    if (_failed > 0) process.exit(1);
}

runAll();
