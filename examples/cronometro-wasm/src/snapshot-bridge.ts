// snapshot-bridge.ts — Bridge between the per-spec WASM shim and the demo UI.
//
// The shim emitted by trenza-cli (cronometro-wasm-shim) exposes a single
// SystemWasm class with three methods:
//   - new()
//   - dispatch(event, payload_json)  -> JSON string
//   - snapshot()                     -> JSON string
//
// The dispatch result is a "rich snapshot" that carries:
//   { base, overlay_stack, concurrent, current, triggered_effects }
//
// Where `triggered_effects` is a list of Rust-Debug-formatted strings of the
// shape `name("arg0", true, 5)`. We parse each one and re-route it to the
// existing effectsObj from main.ts, preserving the contract the demo already
// expects (function name + positional args).
//
// This file replaces the old TrenzaSystem (which wrapped the legacy generic
// InterpreterWasm). The class exposed here matches the public surface used
// by main.ts/overlays.ts:  current_state, concurrent_states, dispatch.

import init, { SystemWasm } from '../wasm-shim/pkg/cronometro_wasm_shim.js';
import { Contexto } from './CronometroPSP_out';

// ── Effect-call parsing ───────────────────────────────────────────────────

/**
 * Parses a Rust-Debug effect record like:
 *   `actualizarComentario("hola")`
 *   `iniciar_sesion("t1", "nota", 5, false)`
 *   `cerrar`                                   (no args)
 */
export function parseEffectCall(s: string): { name: string; args: unknown[] } {
  const openParen = s.indexOf('(');
  if (openParen === -1) {
    return { name: s.trim(), args: [] };
  }
  const name = s.slice(0, openParen).trim();
  // Strip trailing ')'. Defensive: tolerate trailing whitespace.
  let body = s.slice(openParen + 1).trimEnd();
  if (body.endsWith(')')) body = body.slice(0, -1);
  if (body.length === 0) return { name, args: [] };
  const args = splitTopLevelArgs(body).map(parseDebugToken);
  return { name, args };
}

/** Split a "(...)" body on commas that live outside string literals. */
function splitTopLevelArgs(s: string): string[] {
  const out: string[] = [];
  let cur = '';
  let inStr = false;
  let escape = false;
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    if (escape) { cur += c; escape = false; continue; }
    if (inStr) {
      if (c === '\\') { cur += c; escape = true; continue; }
      if (c === '"')  { cur += c; inStr = false; continue; }
      cur += c;
      continue;
    }
    if (c === '"') { cur += c; inStr = true; continue; }
    if (c === ',') { out.push(cur); cur = ''; continue; }
    cur += c;
  }
  if (cur.length > 0) out.push(cur);
  return out;
}

/** Convert one Rust-Debug token into a JS value (best effort). */
function parseDebugToken(raw: string): unknown {
  const s = raw.trim();
  if (s === 'true')  return true;
  if (s === 'false') return false;
  if (s === 'None')  return null;
  // Quoted string — Rust Debug for &str/String is JSON-compatible enough.
  if (s.startsWith('"') && s.endsWith('"')) {
    try { return JSON.parse(s); }
    catch { return s.slice(1, -1); }
  }
  // Numeric (i32/u64/f64).
  if (/^-?\d+(\.\d+)?$/.test(s)) {
    const n = Number(s);
    if (!Number.isNaN(n)) return n;
  }
  // Some(...) / Ok(...) — strip the wrapper, recurse on the inner token.
  const wrapMatch = s.match(/^(Some|Ok)\((.*)\)$/);
  if (wrapMatch) return parseDebugToken(wrapMatch[2]);
  // Fallback: hand back the raw token; the receiving effect can decide.
  return s;
}

// ── System wrapper ────────────────────────────────────────────────────────

interface RichSnapshot {
  base: string;
  overlay_stack: string[];
  concurrent: string[];
  current: string;
  triggered_effects?: string[];
}

/**
 * Drop-in replacement for the legacy TrenzaSystem. main.ts can keep using
 *   system.current_state, system.concurrent_states, system.dispatch(ev, p)
 * verbatim.
 */
export class TrenzaSystem {
  private inner: SystemWasm;
  private effects: Record<string, (...args: unknown[]) => void>;

  public current_state: Contexto;
  public concurrent_states: Set<Contexto>;
  public overlay_stack: Contexto[];
  public base: Contexto;

  constructor(inner: SystemWasm, effects: Record<string, (...args: unknown[]) => void>) {
    this.inner = inner;
    this.effects = effects;
    const snap = JSON.parse(this.inner.snapshot()) as RichSnapshot;
    this.base            = snap.base    as Contexto;
    this.current_state   = snap.current as Contexto;
    this.overlay_stack   = snap.overlay_stack as Contexto[];
    this.concurrent_states = new Set(snap.concurrent as Contexto[]);
  }

  dispatch(event: string, payload: unknown = {}): void {
    const json = this.inner.dispatch(event, JSON.stringify(payload ?? {}));
    let snap: RichSnapshot;
    try {
      snap = JSON.parse(json) as RichSnapshot;
    } catch (e) {
      console.error('[bridge] failed to parse snapshot JSON', e, json);
      return;
    }
    this.base              = snap.base    as Contexto;
    this.current_state     = snap.current as Contexto;
    this.overlay_stack     = snap.overlay_stack as Contexto[];
    this.concurrent_states = new Set(snap.concurrent as Contexto[]);

    for (const callStr of snap.triggered_effects ?? []) {
      const { name, args } = parseEffectCall(callStr);
      const fn = this.effects[name];
      if (typeof fn !== 'function') {
        console.warn(`[bridge] no handler for effect ${name}`, args);
        continue;
      }
      try { fn(...args); }
      catch (e) { console.error(`[bridge] effect ${name} threw`, e); }
    }
  }
}

// ── Init ──────────────────────────────────────────────────────────────────

export async function createTrenzaSystem(
  effects: Record<string, (...args: unknown[]) => void>,
): Promise<TrenzaSystem> {
  await init();
  return new TrenzaSystem(new SystemWasm(), effects);
}
