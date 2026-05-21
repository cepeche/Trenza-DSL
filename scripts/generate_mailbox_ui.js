#!/usr/bin/env node
// Genera history/coordination/index.html con el timeline del buzon
// inter-agente. Sin dependencias externas; pensado para ejecutarse desde
// post-commit en cualquier plataforma con Node instalado.

const fs = require('fs');
const path = require('path');

const REPO_ROOT = path.resolve(__dirname, '..');
const COORD_DIR = path.join(REPO_ROOT, 'history', 'coordination');
const OUTPUT = path.join(COORD_DIR, 'index.html');

const AGENT_COLORS = {
  'CL-Code':        '#7c3aed',
  'CL-Antigravity': '#0ea5e9',
  'CL':             '#6366f1',
  'GE':             '#16a34a',
  'HUMAN':          '#ea580c',
};

function parseFrontmatter(text) {
  const m = text.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!m) return { meta: {}, body: text };
  const meta = {};
  for (const line of m[1].split(/\r?\n/)) {
    const kv = line.match(/^([a-zA-Z_]+):\s*(.*)$/);
    if (!kv) continue;
    let v = kv[2].trim();
    if (v === 'true') v = true;
    else if (v === 'false') v = false;
    else if (v === 'null' || v === '') v = null;
    else if (/^\d+$/.test(v)) v = parseInt(v, 10);
    meta[kv[1]] = v;
  }
  return { meta, body: m[2] };
}

function collectMessages() {
  const messages = [];
  const walk = (dir, status) => {
    if (!fs.existsSync(dir)) return;
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(full, status);
      } else if (entry.name.endsWith('.md') && entry.name !== 'README.md') {
        const text = fs.readFileSync(full, 'utf8');
        const { meta, body } = parseFrontmatter(text);
        if (!meta.from || !meta.to) continue;
        messages.push({
          status,
          path: path.relative(REPO_ROOT, full).replace(/\\/g, '/'),
          filename: entry.name,
          meta,
          body: body.trim(),
        });
      }
    }
  };
  walk(path.join(COORD_DIR, 'inbox'), 'unread');
  walk(path.join(COORD_DIR, 'archive'), 'archived');
  messages.sort((a, b) => a.filename.localeCompare(b.filename));
  return messages;
}

function escapeHtml(s) {
  return String(s)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function renderBodyExcerpt(body, maxLines = 8) {
  const lines = body.split(/\r?\n/).slice(0, maxLines);
  const truncated = body.split(/\r?\n/).length > maxLines;
  let html = lines.map(escapeHtml).join('<br>');
  if (truncated) html += '<br><em>… (truncado)</em>';
  return html;
}

function render(messages) {
  const agents = [...new Set(messages.flatMap(m => [m.meta.from, m.meta.to]))].sort();
  const threads = [...new Set(messages.map(m => m.meta.thread))].sort();
  const updated = new Date().toISOString();

  const cards = messages.map(m => {
    const c = AGENT_COLORS[m.meta.from] || '#64748b';
    const badges = [];
    if (m.status === 'unread') badges.push('<span class="b unread">UNREAD</span>');
    if (m.meta.closes) badges.push('<span class="b closes">CLOSES</span>');
    if (m.meta.requires_reply) badges.push('<span class="b reply">REPLY</span>');
    if (m.meta.deadline) badges.push(`<span class="b deadline">⏱ ${escapeHtml(m.meta.deadline)}</span>`);
    return `
<article class="msg" data-from="${escapeHtml(m.meta.from)}" data-to="${escapeHtml(m.meta.to)}" data-thread="${escapeHtml(m.meta.thread)}" data-status="${m.status}">
  <header>
    <span class="agent" style="background:${c}">${escapeHtml(m.meta.from)}</span>
    <span class="arrow">→</span>
    <span class="agent">${escapeHtml(m.meta.to)}</span>
    <span class="thread">${escapeHtml(m.meta.thread)}</span>
    <span class="seq">seq ${escapeHtml(m.meta.seq)}</span>
    ${badges.join('')}
  </header>
  <div class="body">${renderBodyExcerpt(m.body)}</div>
  <footer><a href="../../${escapeHtml(m.path)}">${escapeHtml(m.path)}</a></footer>
</article>`;
  }).join('\n');

  return `<!doctype html>
<html lang="es">
<head>
<meta charset="utf-8">
<title>Trenza Mailbox — Coordinación inter-agente</title>
<style>
  * { box-sizing: border-box; }
  body { font-family: -apple-system, Segoe UI, sans-serif; margin: 0; background: #0f172a; color: #e2e8f0; }
  header.top { padding: 1rem 2rem; background: #1e293b; border-bottom: 1px solid #334155; }
  h1 { margin: 0; font-size: 1.2rem; }
  .meta { color: #94a3b8; font-size: 0.85rem; margin-top: 0.3rem; }
  .filters { padding: 0.8rem 2rem; background: #1e293b; border-bottom: 1px solid #334155; display: flex; gap: 1rem; flex-wrap: wrap; align-items: center; }
  .filters label { font-size: 0.85rem; color: #94a3b8; }
  .filters select, .filters button { background: #334155; color: #e2e8f0; border: 1px solid #475569; padding: 0.3rem 0.6rem; border-radius: 4px; font-size: 0.85rem; }
  .filters button { cursor: pointer; }
  main { padding: 1.5rem 2rem; max-width: 1100px; margin: 0 auto; }
  .msg { background: #1e293b; border: 1px solid #334155; border-radius: 8px; margin-bottom: 1rem; overflow: hidden; }
  .msg[data-status="unread"] { border-color: #f59e0b; box-shadow: 0 0 0 1px #f59e0b inset; }
  .msg header { padding: 0.7rem 1rem; background: #0f172a; display: flex; gap: 0.6rem; align-items: center; flex-wrap: wrap; }
  .agent { background: #475569; color: white; padding: 0.15rem 0.55rem; border-radius: 4px; font-size: 0.8rem; font-weight: 600; }
  .arrow { color: #94a3b8; }
  .thread { color: #cbd5e1; font-family: monospace; font-size: 0.85rem; }
  .seq { color: #94a3b8; font-size: 0.8rem; }
  .b { font-size: 0.7rem; padding: 0.1rem 0.4rem; border-radius: 3px; font-weight: 600; letter-spacing: 0.03em; }
  .b.unread { background: #f59e0b; color: #422006; }
  .b.closes { background: #ef4444; color: white; }
  .b.reply { background: #3b82f6; color: white; }
  .b.deadline { background: #334155; color: #cbd5e1; }
  .body { padding: 0.8rem 1rem; font-size: 0.9rem; line-height: 1.5; }
  .msg footer { padding: 0.4rem 1rem; background: #0f172a; font-size: 0.75rem; }
  .msg footer a { color: #64748b; text-decoration: none; font-family: monospace; }
  .msg footer a:hover { color: #cbd5e1; }
  .empty { color: #64748b; text-align: center; padding: 2rem; font-style: italic; }
</style>
</head>
<body>
<header class="top">
  <h1>🧵 Trenza Mailbox — Coordinación inter-agente</h1>
  <div class="meta">Generado: ${updated} · ${messages.length} mensajes (${messages.filter(m => m.status === 'unread').length} sin leer)</div>
</header>
<div class="filters">
  <label>Agente:
    <select id="agent">
      <option value="">(todos)</option>
      ${agents.map(a => `<option value="${escapeHtml(a)}">${escapeHtml(a)}</option>`).join('')}
    </select>
  </label>
  <label>Hilo:
    <select id="thread">
      <option value="">(todos)</option>
      ${threads.map(t => `<option value="${escapeHtml(t)}">${escapeHtml(t)}</option>`).join('')}
    </select>
  </label>
  <label>Estado:
    <select id="status">
      <option value="">(todos)</option>
      <option value="unread">Sin leer</option>
      <option value="archived">Archivados</option>
    </select>
  </label>
  <button onclick="resetFilters()">Limpiar</button>
</div>
<main id="list">
  ${messages.length ? cards : '<div class="empty">No hay mensajes todavía.</div>'}
</main>
<script>
  const $ = id => document.getElementById(id);
  function apply() {
    const a = $('agent').value, t = $('thread').value, s = $('status').value;
    for (const m of document.querySelectorAll('.msg')) {
      const okA = !a || m.dataset.from === a || m.dataset.to === a;
      const okT = !t || m.dataset.thread === t;
      const okS = !s || m.dataset.status === s;
      m.style.display = (okA && okT && okS) ? '' : 'none';
    }
  }
  function resetFilters() {
    $('agent').value = ''; $('thread').value = ''; $('status').value = ''; apply();
  }
  ['agent', 'thread', 'status'].forEach(id => $(id).addEventListener('change', apply));
</script>
</body>
</html>
`;
}

function main() {
  if (!fs.existsSync(COORD_DIR)) {
    console.error(`No existe ${COORD_DIR}; abortando.`);
    process.exit(0);
  }
  const messages = collectMessages();
  const html = render(messages);
  fs.writeFileSync(OUTPUT, html);
  console.log(`[mailbox-ui] ${messages.length} mensajes → ${path.relative(REPO_ROOT, OUTPUT)}`);
}

main();
