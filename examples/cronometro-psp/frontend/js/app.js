/**
 * Aplicación principal - Mi Cronómetro PSP
 * Adaptada para consumir API backend (backend/api/)
 *
 * TABLA DE CONTENIDO — buscar con "// === NombreSección ==="
 * ─────────────────────────────────────────────────────────────────
 * AppState               Estado global (actividades, sesiones, flags UI)
 * inicializarApp         Arranque: carga iconos, datos, UI y timer
 * cargarDatosIniciales   API: getActividades, getTiposTarea, getSesiones
 *
 * § FUNCIONES DE UTILIDAD  getTareasPorActividad, getTiempoHoy*, formatearDuracion
 * § RENDERIZADO            renderTipoTarea/renderTarea, generarPestanas, actualizarTimer
 * § ACCIONES               seleccionarTipoTarea, iniciarTarea, confirmarInicio
 * § SELECTOR ACTIVIDAD     #activityModal (al iniciar tarea con >1 actividad)
 * § SELECTOR ICONOS        #iconPickerCreate / #iconPickerEdit
 * § MODAL CREAR TAREA      #createTaskModal → POST /api/tipos-tarea
 * § GESTIÓN TÁCTIL         umbral 10px para distinguir scroll de tap en móvil
 * § MODO EDICIÓN           body.edit-mode, #editModeButton
 * § MODAL EDITAR TAREA     #editTaskModal → PATCH /api/tipos-tarea
 * § MODAL EDITAR ACTIV.    #editActivityModal → PUT /api/actividades?id=
 * § MODAL CREAR ACTIV.     #createActivityModal → POST /api/actividades
 * § EVENT LISTENERS        cierre de modales al clic en overlay
 * § UI HELPERS             mostrarLoading, mostrarError, toast .toast-aviso
 * § MODAL ACERCA DE        #aboutModal → GET /api/health + /api/acumulado
 * § MODAL PUESTA A CERO    #resetModal (3 fases) → POST /api/reset
 * § MODAL HISTORIAL        #historialModal → GET /api/historial?dias=N
 * ─────────────────────────────────────────────────────────────────
 */

// Estado global de la aplicación
const AppState = {
    actividades: {},
    tiposTarea: [],
    tareas: [],
    sesionesHoy: [],
    sesionActiva: null,
    notasActivas: null,       // notas de la sesión activa actual
    tareaIdPendiente: null,   // tareaId esperando confirmación de comentario
    timerInterval: null,
    pestanaActual: 'frecuentes',
    modoEdicion: false,
    iconoSeleccionado: 'draft',
    iconoEditSeleccionado: 'draft',
    colorSeleccionado: '#667eea',
    colorEditSeleccionado: '#667eea',
    todosIconos: [],    // cargado desde /fonts/material-icons.json
    coloresDisponibles: ['#667eea', '#764ba2', '#f093fb', '#fa709a', '#4facfe', '#00f2fe', '#43e97b', '#38f9d7', '#fa8231', '#feca57']
};

// === INICIALIZACIÓN ===

async function inicializarApp() {
    try {
        mostrarLoading();

        // Cargar lista de iconos Material Symbols
        try {
            const resp = await fetch('fonts/material-icons.json');
            AppState.todosIconos = await resp.json();
        } catch(e) {
            AppState.todosIconos = ['draft','home','settings','search','person','star','favorite','check','close','add','edit','delete'];
        }

        // Cargar datos desde el backend
        await cargarDatosIniciales();

        // Configurar event listeners
        configurarEventListeners();

        // Renderizar UI
        actualizarUI();

        // Iniciar timer (resolución de minutos)
        AppState.timerInterval = setInterval(() => {
            actualizarTimer();
            actualizarTotalHoy();
        }, 60000);

        ocultarLoading();
    } catch (error) {
        console.error('Error al inicializar app:', error);
        mostrarError('Error al cargar la aplicación. Verifica que el backend esté ejecutándose.');
    }
}

async function cargarDatosIniciales() {
    // Cargar actividades
    const actividades = await api.getActividades();
    AppState.actividades = {};
    actividades.forEach(act => {
        AppState.actividades[act.id] = act;
    });

    // Generar pestañas de actividades dinámicamente
    generarPestanasActividades(actividades);

    // Cargar tipos de tarea
    AppState.tiposTarea = await api.getTiposTarea();

    // Generar tareas expandidas (tipo × actividad)
    AppState.tareas = AppState.tiposTarea.flatMap(tipo =>
        tipo.actividades_permitidas.map(actId => ({
            id: `${tipo.id}_${actId}`,
            tipoTareaId: tipo.id,
            actividadId: actId,
            nombre: tipo.nombre,
            icono: tipo.icono,
            usos7d: tipo.usos_7d
        }))
    );

    // Cargar sesiones de hoy
    AppState.sesionesHoy = await api.getSesiones('hoy');

    // Cargar sesión activa
    const sesionActiva = await api.getSesionActiva();
    if (sesionActiva) {
        const clockOffset = sesionActiva.server_time
            ? (sesionActiva.server_time * 1000 - Date.now())
            : 0;
        const inicioMs = sesionActiva.inicio * 1000 - clockOffset;
        AppState.sesionActiva = {
            tareaId: sesionActiva.tarea_id,
            inicio: new Date(Math.floor(inicioMs / 60000) * 60000)
        };
        AppState.notasActivas = sesionActiva.notas || null;
    }
}

// === FUNCIONES DE UTILIDAD ===
// getTareasPorActividad, getTareasFrecuentes — filtrado y ordenación
// getTiempoHoyTipoTarea, getTiempoHoyTarea   — acumula sesiones cerradas + activa (hoy)
// formatearDuracion(seg) → "H:MM"

function getTareasPorActividad(actividadId) {
    return AppState.tareas
        .filter(t => t.actividadId === actividadId)
        .sort((a, b) => b.usos7d - a.usos7d);
}

function getTareasFrecuentes() {
    // Tipos de tarea recientes (de sesionesHoy)
    const tiposRecientesIds = [...new Set(
        AppState.sesionesHoy.slice(-3).map(s => s.tipo_tarea_id).filter(Boolean)
    )];

    // Top 8 tipos con usos_7d > 0 (solo los que tienen historial real)
    const topTipos = [...AppState.tiposTarea]
        .filter(t => t.usos_7d > 0)
        .sort((a, b) => b.usos_7d - a.usos_7d)
        .slice(0, 8);

    const tiposRecientes = tiposRecientesIds
        .map(id => AppState.tiposTarea.find(t => t.id === id))
        .filter(t => t && !topTipos.find(tt => tt.id === t.id));

    return [...topTipos, ...tiposRecientes];
}

function getTiempoHoyTipoTarea(tipoTareaId) {
    // Sumar tiempo de sesiones cerradas de este tipo
    let segundos = AppState.sesionesHoy
        .filter(s => s.tipo_tarea_id === tipoTareaId && s.duracion)
        .reduce((sum, s) => sum + Number(s.duracion), 0);

    // Si hay sesión activa de este tipo, sumar tiempo transcurrido (solo desde inicio del día)
    if (AppState.sesionActiva) {
        const tareaActiva = AppState.tareas.find(t => t.id === AppState.sesionActiva.tareaId);
        if (tareaActiva && tareaActiva.tipoTareaId === tipoTareaId) {
            const inicioDia = new Date();
            inicioDia.setHours(0, 0, 0, 0);
            const desde = Math.max(AppState.sesionActiva.inicio.getTime(), inicioDia.getTime());
            const transcurrido = Math.floor((Date.now() - desde) / 1000);
            segundos += transcurrido;
        }
    }

    return segundos;
}

function getTiempoHoyTarea(tareaId) {
    // Sumar tiempo de sesiones cerradas de esta tarea
    let segundos = AppState.sesionesHoy
        .filter(s => s.tarea_id === tareaId && s.duracion)
        .reduce((sum, s) => sum + Number(s.duracion), 0);

    // Si esta tarea está activa, sumar tiempo transcurrido (solo desde inicio del día)
    if (AppState.sesionActiva && AppState.sesionActiva.tareaId === tareaId) {
        const inicioDia = new Date();
        inicioDia.setHours(0, 0, 0, 0);
        const desde = Math.max(AppState.sesionActiva.inicio.getTime(), inicioDia.getTime());
        const transcurrido = Math.floor((Date.now() - desde) / 1000);
        segundos += transcurrido;
    }

    return segundos;
}

function formatearDuracion(segundos) {
    const horas = Math.floor(segundos / 3600);
    const minutos = Math.floor((segundos % 3600) / 60);
    return `${horas}:${minutos.toString().padStart(2, '0')}`;
}

// === RENDERIZADO ===
// renderTipoTarea / renderTarea / renderGrid — generan HTML de tarjetas
// generarPestanasActividades — recrea tabs DOM + listeners (llamado en cargarDatosIniciales)
// actualizarTimer / actualizarTotalHoy — actualiza header y pie cada 60s
// actualizarUI — re-renderiza todos los grids + timer + total

function renderTipoTarea(tipoTarea) {
    const tiempoHoy = getTiempoHoyTipoTarea(tipoTarea.id);

    // Verificar si alguna tarea de este tipo está activa
    let esActiva = false;
    if (AppState.sesionActiva) {
        const tareaActiva = AppState.tareas.find(t => t.id === AppState.sesionActiva.tareaId);
        esActiva = tareaActiva && tareaActiva.tipoTareaId === tipoTarea.id;
    }

    return `
        <div class="task-card ${esActiva ? 'active' : ''}"
             ontouchstart="tarjetaTouchStart(event)"
             ontouchmove="tarjetaTouchMove(event)"
             ontouchend="tarjetaTouchEnd(event,'${tipoTarea.id}',true)"
             onclick="tarjetaClick('${tipoTarea.id}',true)">
            <div class="task-icon"><span class="material-symbols-outlined">${tipoTarea.icono}</span></div>
            <div class="task-name">${tipoTarea.nombre}</div>
            <div class="task-time">${formatearDuracion(tiempoHoy)}</div>
        </div>
    `;
}

function renderTarea(tarea) {
    const tiempoHoy = getTiempoHoyTarea(tarea.id);
    const esActiva = AppState.sesionActiva && AppState.sesionActiva.tareaId === tarea.id;

    return `
        <div class="task-card ${esActiva ? 'active' : ''}"
             data-tarea-id="${tarea.id}"
             ontouchstart="tarjetaTouchStart(event)"
             ontouchmove="tarjetaTouchMove(event)"
             ontouchend="tarjetaTouchEnd(event,'${tarea.tipoTareaId}',false,'${tarea.id}')"
             onclick="tarjetaClick('${tarea.tipoTareaId}',false,'${tarea.id}')">
            <div class="task-icon"><span class="material-symbols-outlined">${tarea.icono}</span></div>
            <div class="task-name">${tarea.nombre}</div>
            <div class="task-time">${formatearDuracion(tiempoHoy)}</div>
        </div>
    `;
}

function renderGrid(containerId, items, esTipoTarea = false) {
    const container = document.getElementById(containerId);
    if (!container) return;

    if (items.length === 0) {
        const msg = esTipoTarea
            ? 'Usa el botón + para crear tareas y empezarán a aparecer aquí'
            : 'No hay tareas en esta actividad';
        container.innerHTML = `<div class="empty-state">${msg}</div>`;
        return;
    }

    if (esTipoTarea) {
        container.innerHTML = items.map(renderTipoTarea).join('');
    } else {
        container.innerHTML = items.map(renderTarea).join('');
    }
}

function generarPestanasActividades(actividades) {
    const tabsContainer  = document.getElementById('tabs');
    const tabsContent    = document.getElementById('tabsContent');

    // Eliminar pestañas y contenidos de actividades anteriores (no la de Frecuentes)
    tabsContainer.querySelectorAll('.tab[data-tab]:not([data-tab="frecuentes"])').forEach(el => el.remove());
    tabsContent.querySelectorAll('.tab-content:not([data-tab="frecuentes"])').forEach(el => el.remove());

    actividades.forEach(act => {
        // Pestaña
        const tab = document.createElement('button');
        tab.className = 'tab';
        tab.dataset.tab = act.id;
        tab.textContent = act.nombre;
        tab.addEventListener('click', () => {
            if (AppState.modoEdicion) {
                mostrarModalEditarActividad(act.id);
                return;
            }
            AppState.pestanaActual = act.id;
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            document.querySelector(`.tab-content[data-tab="${act.id}"]`).classList.add('active');
            renderGrid(`grid-${act.id}`, getTareasPorActividad(act.id));
        });
        tabsContainer.appendChild(tab);

        // Contenido
        const content = document.createElement('div');
        content.className = 'tab-content';
        content.dataset.tab = act.id;
        content.innerHTML = `<div class="tasks-grid" id="grid-${act.id}"></div>`;
        tabsContent.appendChild(content);
    });

    // Restaurar pestaña activa si era una actividad (generarPestañas la destruye y recrea sin .active)
    if (AppState.pestanaActual !== 'frecuentes') {
        tabsContainer.querySelector('[data-tab="frecuentes"]')?.classList.remove('active');
        document.querySelector('.tab-content[data-tab="frecuentes"]')?.classList.remove('active');
        const actTab = tabsContainer.querySelector(`[data-tab="${AppState.pestanaActual}"]`);
        const actContent = document.querySelector(`.tab-content[data-tab="${AppState.pestanaActual}"]`);
        if (actTab) actTab.classList.add('active');
        if (actContent) actContent.classList.add('active');
    }

    // Re-registrar listener de la pestaña Frecuentes
    const tabFrecuentes = tabsContainer.querySelector('[data-tab="frecuentes"]');
    // Clonar para eliminar listeners viejos
    const tabFrecuentesNuevo = tabFrecuentes.cloneNode(true);
    tabFrecuentes.replaceWith(tabFrecuentesNuevo);
    tabFrecuentesNuevo.addEventListener('click', () => {
        if (AppState.modoEdicion) return; // Frecuentes no tiene edición propia; ignorar en modo edición
        AppState.pestanaActual = 'frecuentes';
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        tabFrecuentesNuevo.classList.add('active');
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        document.querySelector('.tab-content[data-tab="frecuentes"]').classList.add('active');
        renderGrid('gridFrecuentes', getTareasFrecuentes(), true);
    });
}

function actualizarTimer() {
    if (!AppState.sesionActiva) {
        document.getElementById('timerDisplay').textContent = '--:--';
        document.getElementById('activeTaskName').textContent = 'Ninguna tarea activa';
        return;
    }

    const tarea = AppState.tareas.find(t => t.id === AppState.sesionActiva.tareaId);
    const actividad = AppState.actividades[tarea.actividadId];

    const transcurrido = Math.floor((Date.now() - AppState.sesionActiva.inicio) / 60000);
    const horas = Math.floor(transcurrido / 60);
    const minutos = transcurrido % 60;

    document.getElementById('timerDisplay').textContent =
        `${horas.toString().padStart(2, '0')}:${minutos.toString().padStart(2, '0')}`;

    document.getElementById('activeTaskName').innerHTML =
        `<span class="material-symbols-outlined" style="font-size:1rem;vertical-align:middle;margin-right:0.3rem">${tarea.icono}</span>${tarea.nombre} • ${actividad.nombre}`;
}

function actualizarTotalHoy() {
    let totalSegundos = AppState.sesionesHoy
        .filter(s => s.duracion)
        .reduce((sum, s) => sum + Number(s.duracion), 0);

    if (AppState.sesionActiva) {
        const inicioDia = new Date();
        inicioDia.setHours(0, 0, 0, 0);
        const desde = Math.max(AppState.sesionActiva.inicio.getTime(), inicioDia.getTime());
        const transcurrido = Math.floor((Date.now() - desde) / 1000);
        totalSegundos += transcurrido;
    }

    document.getElementById('totalToday').textContent = 'Hoy: ' + formatearDuracion(totalSegundos);

    // Comentario de la sesión activa
    const comentarioEl = document.getElementById('bottomComment');
    comentarioEl.textContent = AppState.notasActivas || '';
}

function actualizarUI() {
    // Renderizar pestaña frecuentes
    renderGrid('gridFrecuentes', getTareasFrecuentes(), true);

    // Renderizar pestañas de actividades
    Object.keys(AppState.actividades).forEach(actId => {
        const gridId = `grid-${actId}`;
        const grid = document.getElementById(gridId);
        if (grid) {
            renderGrid(gridId, getTareasPorActividad(actId));
        }
    });

    actualizarTimer();
    actualizarTotalHoy();
}

// === ACCIONES ===
// Flujo inicio de sesión: seleccionarTipoTarea → (si >1 actividad) mostrarModalActividades
//   → iniciarTarea → #commentModal → confirmarInicioConComentario → api.iniciarSesion
// API: POST /api/sesiones | Estado: sesionActiva, notasActivas, tareaIdPendiente

function seleccionarTipoTarea(tipoTareaId) {
    const tipoTarea = AppState.tiposTarea.find(t => t.id === tipoTareaId);

    // Si solo hay una actividad permitida, iniciar directamente
    if (tipoTarea.actividades_permitidas.length === 1) {
        const tareaId = `${tipoTareaId}_${tipoTarea.actividades_permitidas[0]}`;
        iniciarTarea(tareaId);
        return;
    }

    // Si hay múltiples actividades, mostrar modal
    mostrarModalActividades(tipoTarea);
}

function iniciarTarea(tareaId) {
    if (AppState.modoEdicion) return; // defensivo: no iniciar en modo edición
    // Guardar tarea pendiente y abrir modal de comentario
    const tarea = AppState.tareas.find(t => t.id === tareaId);
    AppState.tareaIdPendiente = tareaId;
    document.getElementById('commentModalIcon').innerHTML = tarea
        ? `<span class="material-symbols-outlined">${tarea.icono}</span>` : '';
    document.getElementById('commentModalName').textContent = tarea ? tarea.nombre : '';
    document.getElementById('commentInput').value = '';
    document.getElementById('retroactivoInput').value = '0';

    // Mostrar checkbox "Sustituye a la tarea en curso" solo si hay sesión activa
    const sustituirGroup = document.getElementById('sustituirGroup');
    const sustituirCheck = document.getElementById('sustituirCheck');
    sustituirCheck.checked = false;
    sustituirGroup.style.display = AppState.sesionActiva ? 'block' : 'none';

    document.getElementById('commentModal').classList.add('active');
    setTimeout(() => document.getElementById('commentInput').focus(), 100);
}

async function confirmarInicioConComentario() {
    const tareaId = AppState.tareaIdPendiente;
    const notas = document.getElementById('commentInput').value.trim() || null;
    const minutosRetroactivos = Math.max(0, parseInt(document.getElementById('retroactivoInput').value, 10) || 0);
    const sustituir = document.getElementById('sustituirCheck').checked;
    cerrarModalComentario();
    try {
        const response = await api.iniciarSesion(tareaId, notas, minutosRetroactivos, sustituir);

        // Actualizar estado local
        const clockOffset = response.data.server_time
            ? (response.data.server_time * 1000 - Date.now())
            : 0;
        const inicioMs = response.data.inicio * 1000 - clockOffset;
        AppState.sesionActiva = {
            tareaId: tareaId,
            inicio: new Date(Math.floor(inicioMs / 60000) * 60000)
        };
        AppState.notasActivas = notas;

        // Recargar sesiones
        AppState.sesionesHoy = await api.getSesiones('hoy');

        actualizarUI();

        // Aviso si se aplicaron menos minutos de los solicitados (solo en modo normal)
        if (!sustituir && minutosRetroactivos > 0) {
            const minutosAplicados = response.data.minutos_aplicados;
            if (minutosAplicados !== null && minutosAplicados !== undefined
                && minutosAplicados < minutosRetroactivos) {
                const motivo = AppState.sesionesHoy.length > 0
                    ? 'había una sesión anterior'
                    : 'es antes de medianoche';
                mostrarAviso(
                    `⚠️ Se aplicaron solo ${minutosAplicados} min de los ${minutosRetroactivos} solicitados (límite: ${motivo}).`
                );
            }
        }
    } catch (error) {
        console.error('Error al iniciar sesión:', error);
        mostrarError('Error al iniciar sesión');
    }
}

function cerrarModalComentario() {
    AppState.tareaIdPendiente = null;
    document.getElementById('commentModal').classList.remove('active');
}

// === SELECTOR DE ACTIVIDAD ===
// HTML: #activityModal | Se muestra cuando un tipo de tarea tiene >1 actividad permitida

function mostrarModalActividades(tipoTarea) {
    const modal = document.getElementById('activityModal');
    const modalIcon = document.getElementById('modalTaskIcon');
    const modalName = document.getElementById('modalTaskName');
    const buttonsContainer = document.getElementById('activityButtons');

    modalIcon.innerHTML = `<span class="material-symbols-outlined">${tipoTarea.icono}</span>`;
    modalName.textContent = tipoTarea.nombre;

    const botonesHTML = tipoTarea.actividades_permitidas.map(actId => {
        const actividad = AppState.actividades[actId];
        const tareaId = `${tipoTarea.id}_${actId}`;
        return `
            <button class="activity-button" onclick="iniciarTarea('${tareaId}'); cerrarModal();">
                ${actividad.nombre}
            </button>
        `;
    }).join('');

    buttonsContainer.innerHTML = botonesHTML;
    modal.classList.add('active');
}

function cerrarModal() {
    document.getElementById('activityModal').classList.remove('active');
}

function toggleSettingsMenu() {
    const menu = document.getElementById('settingsMenu');
    menu.classList.toggle('active');
}

function mostrarModalCrear() {
    if (AppState.pestanaActual === 'frecuentes') {
        mostrarModalCrearTarea();
    } else {
        mostrarModalCrearTarea(AppState.pestanaActual);
    }
}

// === SELECTOR DE ICONOS MATERIAL SYMBOLS ===
// HTML: #iconPickerCreate, #iconPickerEdit | Estado: iconoSeleccionado, iconoEditSeleccionado
// Fuente: AppState.todosIconos (fonts/material-icons.json) | Muestra hasta 70 iconos filtrados

function renderizarIconPicker(containerId, iconoActual, contexto) {
    const container = document.getElementById(containerId);
    if (!container) return;
    const busqueda = document.getElementById('iconSearch' + (contexto === 'create' ? 'Create' : 'Edit'))?.value.toLowerCase() || '';
    const iconosFiltrados = busqueda
        ? AppState.todosIconos.filter(n => n.includes(busqueda)).slice(0, 70)
        : AppState.todosIconos.slice(0, 70);

    container.innerHTML = iconosFiltrados.map(nombre => `
        <div class="icon-option ${nombre === iconoActual ? 'selected' : ''}"
             onclick="seleccionarIcono('${nombre}','${contexto}')"
             title="${nombre}">
            <span class="material-symbols-outlined">${nombre}</span>
        </div>
    `).join('');
}

function filtrarIconos(contexto) {
    const iconoActual = contexto === 'create' ? AppState.iconoSeleccionado : AppState.iconoEditSeleccionado;
    renderizarIconPicker(contexto === 'create' ? 'iconPickerCreate' : 'iconPickerEdit', iconoActual, contexto);
}

function seleccionarIcono(nombre, contexto) {
    if (contexto === 'create') {
        AppState.iconoSeleccionado = nombre;
        document.getElementById('iconPreviewSymbolCreate').textContent = nombre;
        document.getElementById('iconPreviewNameCreate').textContent = nombre;
        renderizarIconPicker('iconPickerCreate', nombre, 'create');
    } else {
        AppState.iconoEditSeleccionado = nombre;
        document.getElementById('iconPreviewSymbolEdit').textContent = nombre;
        document.getElementById('iconPreviewNameEdit').textContent = nombre;
        renderizarIconPicker('iconPickerEdit', nombre, 'edit');
    }
}

// === MODAL CREAR TAREA ===
// HTML: #createTaskModal | API: POST /api/tipos-tarea
// Estado: iconoSeleccionado | Acceso: botón FAB + (#createActivityModal si actividadPredefinida)

function mostrarModalCrearTarea(actividadPredefinida = null) {
    const modal = document.getElementById('createTaskModal');

    AppState.iconoSeleccionado = 'draft';
    document.getElementById('iconPreviewSymbolCreate').textContent = 'draft';
    document.getElementById('iconPreviewNameCreate').textContent = 'draft';
    document.getElementById('iconSearchCreate').value = '';
    renderizarIconPicker('iconPickerCreate', 'draft', 'create');

    // Renderizar checkboxes de actividades
    const checkboxContainer = document.getElementById('actividadesCheckbox');
    checkboxContainer.innerHTML = Object.values(AppState.actividades).map(act => {
        const checked = actividadPredefinida ? act.id === actividadPredefinida : false;
        return `
            <label class="checkbox-label">
                <input type="checkbox" value="${act.id}" ${checked ? 'checked' : ''}>
                ${act.nombre}
            </label>
        `;
    }).join('');

    modal.classList.add('active');
}

function cerrarModalCrearTarea() {
    const modal = document.getElementById('createTaskModal');
    modal.classList.remove('active');
    document.getElementById('newTaskName').value = '';
    AppState.iconoSeleccionado = 'draft';
}

async function guardarNuevaTarea() {
    const nombre = document.getElementById('newTaskName').value.trim();
    if (!nombre) {
        alert('Por favor ingresa un nombre para la tarea');
        return;
    }

    const actividadesSeleccionadas = Array.from(
        document.querySelectorAll('#actividadesCheckbox input[type="checkbox"]:checked')
    ).map(cb => cb.value);

    if (actividadesSeleccionadas.length === 0) {
        alert('Selecciona al menos una actividad');
        return;
    }

    try {
        await api.crearTipoTarea(nombre, AppState.iconoSeleccionado, actividadesSeleccionadas);
        await cargarDatosIniciales();
        cerrarModalCrearTarea();
        actualizarUI();
        alert(`✅ Tarea "${nombre}" creada`);
    } catch (error) {
        console.error('Error al crear tarea:', error);
        alert('Error al crear tarea: ' + error.message);
    }
}

// === GESTIÓN TÁCTIL CON UMBRAL DE MOVIMIENTO ===
// En móvil, onClick se dispara aunque el dedo se haya deslizado ligeramente.
// Se registra posición inicial (touchstart) y se cancela si movimiento > 10px.
// Vars globales: _touchStartX, _touchStartY, _touchCancelled
// Uso en HTML: ontouchstart/ontouchmove/ontouchend en .task-card

let _touchStartX = 0;
let _touchStartY = 0;
let _touchCancelled = false;

function tarjetaTouchStart(e) {
    _touchStartX = e.touches[0].clientX;
    _touchStartY = e.touches[0].clientY;
    _touchCancelled = false;
}

function tarjetaTouchMove(e) {
    const dx = Math.abs(e.touches[0].clientX - _touchStartX);
    const dy = Math.abs(e.touches[0].clientY - _touchStartY);
    if (dx > 10 || dy > 10) _touchCancelled = true;
}

function tarjetaTouchEnd(e, tipoTareaId, esTipo, tareaId) {
    if (_touchCancelled) return;
    e.preventDefault(); // evitar que dispare también el onclick
    if (AppState.modoEdicion) {
        mostrarModalEditarTarea(tipoTareaId);
    } else if (esTipo) {
        seleccionarTipoTarea(tipoTareaId);
    } else {
        iniciarTarea(tareaId);
    }
}

function tarjetaClick(tipoTareaId, esTipo, tareaId) {
    // Solo se ejecuta en ratón (en táctil, preventDefault en touchend lo evita)
    if (AppState.modoEdicion) {
        mostrarModalEditarTarea(tipoTareaId);
    } else if (esTipo) {
        seleccionarTipoTarea(tipoTareaId);
    } else {
        iniciarTarea(tareaId);
    }
}

// === MODO EDICIÓN ===
// HTML: #editModeButton | CSS: body.edit-mode (fondo rosa, ✏️ en pestañas de actividad)
// Activo: click en tarjeta → MODAL EDITAR TAREA | click en pestaña → MODAL EDITAR ACTIV.

function toggleModoEdicion() {
    AppState.modoEdicion = !AppState.modoEdicion;
    document.getElementById('editModeButton').classList.toggle('active', AppState.modoEdicion);
    document.body.classList.toggle('edit-mode', AppState.modoEdicion);
    actualizarUI();
}

// === MODAL EDITAR TAREA ===
// HTML: #editTaskModal | API: PATCH /api/tipos-tarea
// Estado: iconoEditSeleccionado | Trigger: modo edición + click en tarjeta

function mostrarModalEditarTarea(tipoTareaId) {
    const tipo = AppState.tiposTarea.find(t => t.id === tipoTareaId);
    if (!tipo) return;

    document.getElementById('editTaskId').value = tipoTareaId;
    document.getElementById('editTaskName').value = tipo.nombre;

    AppState.iconoEditSeleccionado = tipo.icono;
    document.getElementById('iconPreviewSymbolEdit').textContent = tipo.icono;
    document.getElementById('iconPreviewNameEdit').textContent = tipo.icono;
    document.getElementById('iconSearchEdit').value = '';
    renderizarIconPicker('iconPickerEdit', tipo.icono, 'edit');

    document.getElementById('editTaskModal').classList.add('active');
}

function cerrarModalEditarTarea() {
    document.getElementById('editTaskModal').classList.remove('active');
}

async function guardarEdicionTarea() {
    const id = document.getElementById('editTaskId').value;
    const nombre = document.getElementById('editTaskName').value.trim();
    const icono = AppState.iconoEditSeleccionado;

    if (!nombre) {
        alert('El nombre no puede estar vacío');
        return;
    }

    try {
        await api.editarTipoTarea(id, nombre, icono);
        await cargarDatosIniciales();
        cerrarModalEditarTarea();
        actualizarUI();
    } catch (error) {
        console.error('Error al editar tarea:', error);
        alert('Error al guardar: ' + error.message);
    }
}

// === MODAL EDITAR ACTIVIDAD ===
// HTML: #editActivityModal | API: PUT /api/actividades?id=
// Estado: colorEditSeleccionado | Trigger: modo edición + click en pestaña de actividad

function mostrarModalEditarActividad(id) {
    const act = AppState.actividades[id];
    if (!act) return;

    document.getElementById('editActivityId').value = id;
    document.getElementById('editActivityName').value = act.nombre;
    AppState.colorEditSeleccionado = act.color;

    const colorPicker = document.getElementById('editColorPicker');
    colorPicker.innerHTML = AppState.coloresDisponibles.map(color =>
        `<div class="color-option ${color === act.color ? 'selected' : ''}"
              style="background: ${color}"
              onclick="seleccionarColorEdit('${color}')"></div>`
    ).join('');

    document.getElementById('editActivityPermanente').checked = Number(act.permanente) === 1;
    document.getElementById('editActivityModal').classList.add('active');
}

function cerrarModalEditarActividad() {
    document.getElementById('editActivityModal').classList.remove('active');
}

function seleccionarColorEdit(color) {
    AppState.colorEditSeleccionado = color;
    document.querySelectorAll('#editColorPicker .color-option').forEach(opt => {
        opt.classList.toggle('selected', opt.style.background === color);
    });
}

async function guardarEdicionActividad() {
    const id = document.getElementById('editActivityId').value;
    const nombre = document.getElementById('editActivityName').value.trim();
    const color = AppState.colorEditSeleccionado;
    const permanente = document.getElementById('editActivityPermanente').checked;

    if (!nombre) {
        alert('El nombre no puede estar vacío');
        return;
    }

    try {
        await api.actualizarActividad(id, { nombre, color, permanente });
        await cargarDatosIniciales();
        cerrarModalEditarActividad();
        actualizarUI();
    } catch (error) {
        console.error('Error al editar actividad:', error);
        alert('Error al guardar: ' + error.message);
    }
}

// === MODAL CREAR ACTIVIDAD ===
// HTML: #createActivityModal | API: POST /api/actividades
// Estado: colorSeleccionado | Acceso: menú ⚙️ → "Nueva actividad"
function mostrarModalCrearActividad() {
    const modal = document.getElementById('createActivityModal');

    // Renderizar color picker
    const colorPicker = document.getElementById('colorPicker');
    colorPicker.innerHTML = AppState.coloresDisponibles.map(color =>
        `<div class="color-option ${color === AppState.colorSeleccionado ? 'selected' : ''}"
              style="background: ${color}"
              onclick="seleccionarColor('${color}')"></div>`
    ).join('');

    modal.classList.add('active');
}

function cerrarModalCrearActividad() {
    const modal = document.getElementById('createActivityModal');
    modal.classList.remove('active');
    document.getElementById('newActivityName').value = '';
    document.getElementById('actividadPermanente').checked = false;
    AppState.colorSeleccionado = '#667eea';
}

function seleccionarColor(color) {
    AppState.colorSeleccionado = color;
    document.querySelectorAll('.color-option').forEach(opt => {
        opt.classList.toggle('selected', opt.style.background === color);
    });
}

async function guardarNuevaActividad() {
    const nombre = document.getElementById('newActivityName').value.trim();
    if (!nombre) {
        alert('Por favor ingresa un nombre para la actividad');
        return;
    }

    const permanente = document.getElementById('actividadPermanente').checked;

    try {
        const response = await api.crearActividad(nombre, AppState.colorSeleccionado, permanente);
        const nuevaActividad = response.data;

        // Si hay tarea activa, vincularla con la nueva actividad
        let mensajeExtra = '';
        if (AppState.sesionActiva) {
            const tareaActiva = AppState.tareas.find(t => t.id === AppState.sesionActiva.tareaId);
            if (tareaActiva) {
                const tipoTarea = AppState.tiposTarea.find(tt => tt.id === tareaActiva.tipoTareaId);
                if (tipoTarea && !tipoTarea.actividades_permitidas.includes(nuevaActividad.id)) {
                    // Aquí deberíamos tener un endpoint para actualizar tipo de tarea
                    // Por ahora solo mostramos el mensaje
                    mensajeExtra = ' (nota: vincúlala manualmente con la tarea activa si es necesario)';
                }
            }
        }

        // Recargar datos
        await cargarDatosIniciales();

        cerrarModalCrearActividad();
        actualizarUI();
        agregarPestanaActividad(nuevaActividad.id, nuevaActividad.nombre);

        alert(`✅ Actividad "${nombre}" creada${mensajeExtra}`);
    } catch (error) {
        console.error('Error al crear actividad:', error);
        alert('Error al crear actividad: ' + error.message);
    }
}

function agregarPestanaActividad(actividadId, nombre) {
    // Las pestañas ya se regeneraron en cargarDatosIniciales.
    // Solo navegamos a la pestaña recién creada.
    const tab = document.querySelector(`.tab[data-tab="${actividadId}"]`);
    if (tab) tab.click();
}

// === EVENT LISTENERS ===
// Cierra modales al clic en el overlay (.modal-overlay): activityModal, createTaskModal,
//   createActivityModal, commentModal, aboutModal, resetModal, editTaskModal, historialModal
// También cierra #settingsMenu al clic fuera del botón

function configurarEventListeners() {
    // Las pestañas tienen sus listeners asignados en generarPestanasActividades()

    // Cerrar modales al hacer click en overlay
    document.getElementById('activityModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModal();
    });

    document.getElementById('createTaskModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalCrearTarea();
    });

    document.getElementById('createActivityModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalCrearActividad();
    });

    document.getElementById('commentModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalComentario();
    });

    document.getElementById('aboutModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalAcercaDe();
    });

    document.getElementById('resetModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalReset();
    });

    document.getElementById('editTaskModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalEditarTarea();
    });

    document.getElementById('historialModal').addEventListener('click', function(e) {
        if (e.target === this) cerrarModalHistorial();
    });

    // Cerrar settings menu al hacer click fuera
    document.addEventListener('click', function(e) {
        const menu = document.getElementById('settingsMenu');
        const button = document.querySelector('.settings-button');
        if (!menu.contains(e.target) && e.target !== button) {
            menu.classList.remove('active');
        }
    });
}

// === UI HELPERS ===
// mostrarLoading / ocultarLoading — spinner durante inicialización (div#loading)
// mostrarError(msg)  — alert bloqueante con prefijo ❌
// mostrarAviso(msg)  — toast .toast-aviso flotante (4s, no bloqueante, animado por CSS)

function mostrarLoading() {
    const loading = document.createElement('div');
    loading.id = 'loading';
    loading.className = 'loading';
    loading.textContent = 'Cargando...';
    document.body.appendChild(loading);
}

function ocultarLoading() {
    const loading = document.getElementById('loading');
    if (loading) loading.remove();
}

function mostrarError(mensaje) {
    alert('❌ ' + mensaje);
}

function mostrarAviso(mensaje) {
    // Toast no bloqueante en la parte superior
    const toast = document.createElement('div');
    toast.className = 'toast-aviso';
    toast.textContent = mensaje;
    document.body.appendChild(toast);
    // Animación de entrada (se hace en CSS con la clase .toast-aviso)
    // Auto-cierre a los 4 segundos
    setTimeout(() => {
        toast.classList.add('toast-salida');
        setTimeout(() => toast.remove(), 400);
    }, 4000);
}

// === MODAL ACERCA DE ===
// HTML: #aboutModal, #aboutContent | API: GET /api/health + GET /api/acumulado
// Muestra: latencia, versión backend, tiempo total acumulado por actividad

async function mostrarModalAcercaDe() {
    document.getElementById('aboutModal').classList.add('active');
    document.getElementById('aboutContent').innerHTML = '<div class="about-loading">Comprobando conexión…</div>';

    try {
        const t0 = Date.now();
        const health = await api.healthCheck();
        const latencia = Date.now() - t0;

        let html = `
            <div class="about-status ok">
                <div class="about-dot"></div>
                <span>Conectado · ${latencia}ms · v${health.version}</span>
            </div>`;

        // Tiempo acumulado por actividad
        const acumulado = await api.getAcumulado();
        if (acumulado.actividades && acumulado.actividades.length > 0) {
            html += '<div class="about-activities">';
            html += '<div class="about-activities-title">Tiempo acumulado</div>';
            acumulado.actividades.forEach(act => {
                const horas = Math.floor(act.tiempo_total / 3600);
                const mins  = Math.floor((act.tiempo_total % 3600) / 60);
                html += `
                    <div class="about-activity-row" style="background:${act.actividad_color}18">
                        <div class="about-activity-name">
                            <div class="about-activity-dot" style="background:${act.actividad_color}"></div>
                            <span>${act.actividad_nombre}</span>
                        </div>
                        <div class="about-activity-time">${horas}:${mins.toString().padStart(2,'0')}</div>
                    </div>`;
            });
            html += '</div>';
        } else {
            html += '<p style="color:#999;font-size:0.9rem;text-align:center;margin-top:1rem">Sin sesiones registradas aún</p>';
        }

        document.getElementById('aboutContent').innerHTML = html;
    } catch (e) {
        document.getElementById('aboutContent').innerHTML = `
            <div class="about-status fail">
                <div class="about-dot"></div>
                <span>Sin conexión con el NAS</span>
            </div>`;
    }
}

function cerrarModalAcercaDe() {
    document.getElementById('aboutModal').classList.remove('active');
}

// === MODAL PUESTA A CERO ===
// HTML: #resetModal → #resetStep1 → #resetStep2 → #resetStep3 (3 fases)
// API: POST /api/reset | Descarga CSV antes de borrar | Var de estado: _resetFase
// Fase 1: aviso | Fase 2: elegir actividades a conservar | Fase 3: confirmar escribiendo BORRAR

let _resetFase = 1;

function exportarCsv() {
    // Descarga directa: el navegador abre la URL y el servidor devuelve el CSV
    const url = API_CONFIG.baseURL + '/api/sesiones?action=exportar';
    const a = document.createElement('a');
    a.href = url;
    a.download = '';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
}

function mostrarModalReset() {
    _resetFase = 1;
    document.getElementById('resetStep1').style.display = 'block';
    document.getElementById('resetStep2').style.display = 'none';
    document.getElementById('resetStep3').style.display = 'none';
    document.getElementById('resetConfirmInput').value = '';
    document.getElementById('btnReset').textContent = 'Continuar';
    document.getElementById('btnReset').disabled = false;
    document.getElementById('btnResetCancel').textContent = 'Cancelar';
    document.getElementById('resetModal').classList.add('active');
}

function cerrarModalReset() {
    document.getElementById('resetModal').classList.remove('active');
}

function retrocederReset() {
    if (_resetFase === 1) {
        cerrarModalReset();
    } else if (_resetFase === 2) {
        _resetFase = 1;
        document.getElementById('resetStep1').style.display = 'block';
        document.getElementById('resetStep2').style.display = 'none';
        document.getElementById('btnReset').textContent = 'Continuar';
        document.getElementById('btnResetCancel').textContent = 'Cancelar';
    } else if (_resetFase === 3) {
        _resetFase = 2;
        document.getElementById('resetStep2').style.display = 'block';
        document.getElementById('resetStep3').style.display = 'none';
        document.getElementById('btnReset').textContent = 'Continuar';
        document.getElementById('btnResetCancel').textContent = '← Atrás';
    }
}

function avanzarReset() {
    if (_resetFase === 1) {
        // Pasar a fase 2: mostrar checkboxes de actividades
        _resetFase = 2;
        document.getElementById('resetStep1').style.display = 'none';
        document.getElementById('resetStep2').style.display = 'block';
        document.getElementById('btnReset').textContent = 'Continuar';
        document.getElementById('btnResetCancel').textContent = '← Atrás';

        // Renderizar lista de actividades
        const lista = document.getElementById('resetActivitiesList');
        lista.innerHTML = Object.values(AppState.actividades).map(act => {
            // Permanente si la BD lo indica, o si tiene usos recientes
            const esPermanente = act.permanente == 1;
            const checked = esPermanente ? 'checked' : '';
            const lockIcon = esPermanente ? '<span class="act-lock" title="Actividad permanente">🔒</span>' : '';
            return `
                <label class="reset-activity-item">
                    <input type="checkbox" value="${act.id}" ${checked}>
                    <span class="act-dot" style="background:${act.color}"></span>
                    <span>${act.nombre}</span>
                    ${lockIcon}
                </label>`;
        }).join('');

    } else if (_resetFase === 2) {
        // Pasar a fase 3: confirmación BORRAR
        _resetFase = 3;
        document.getElementById('resetStep2').style.display = 'none';
        document.getElementById('resetStep3').style.display = 'block';
        document.getElementById('btnReset').textContent = 'Borrar todo';
        document.getElementById('btnResetCancel').textContent = '← Atrás';
        setTimeout(() => document.getElementById('resetConfirmInput').focus(), 100);

    } else {
        ejecutarReset();
    }
}

async function ejecutarReset() {
    const confirmacion = document.getElementById('resetConfirmInput').value.trim();
    if (confirmacion !== 'BORRAR') {
        document.getElementById('resetConfirmInput').style.borderColor = '#e53935';
        setTimeout(() => document.getElementById('resetConfirmInput').style.borderColor = '', 1000);
        return;
    }

    // Recoger IDs de actividades a conservar
    const conservar = Array.from(
        document.querySelectorAll('#resetActivitiesList input[type="checkbox"]:checked')
    ).map(cb => cb.value);

    document.getElementById('btnReset').disabled = true;
    document.getElementById('btnReset').textContent = 'Procesando…';

    try {
        const { blob, filename } = await api.resetear(conservar);

        // Intentar descarga del CSV
        try {
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = filename;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            setTimeout(() => URL.revokeObjectURL(url), 1000);
        } catch (downloadErr) {
            console.warn('No se pudo descargar el CSV automáticamente:', downloadErr);
        }

        // Limpiar estado local y refrescar UI (independientemente de si descargó)
        AppState.sesionActiva = null;
        AppState.notasActivas = null;
        AppState.sesionesHoy = [];

        cerrarModalReset();
        await cargarDatosIniciales();
        actualizarUI();
    } catch (e) {
        console.error('Error en reset:', e);
        mostrarError('Error al realizar la puesta a cero: ' + e.message);
        document.getElementById('btnReset').disabled = false;
        document.getElementById('btnReset').textContent = 'Borrar todo';
    }
}

// === MODAL HISTORIAL ===
// HTML: #historialModal, #historialContenido | API: GET /api/historial?dias=N
// Muestra barras de colores por día + tabla resumen | Var: _historialDias (7 ó 30)

let _historialDias = 7;

async function mostrarModalHistorial() {
    document.getElementById('historialModal').classList.add('active');
    await cargarHistorial(_historialDias);
}

function cerrarModalHistorial() {
    document.getElementById('historialModal').classList.remove('active');
}

async function cambiarPeriodoHistorial(dias) {
    _historialDias = dias;
    document.querySelectorAll('.historial-btn-periodo').forEach(btn => {
        btn.classList.toggle('active', parseInt(btn.dataset.dias) === dias);
    });
    await cargarHistorial(dias);
}

async function cargarHistorial(dias) {
    const contenido = document.getElementById('historialContenido');
    contenido.innerHTML = '<div class="historial-loading">Cargando…</div>';

    try {
        const data = await api.getHistorial(dias);
        contenido.innerHTML = renderHistorial(data, dias);
    } catch (e) {
        contenido.innerHTML = '<div class="historial-loading">Error al cargar el historial</div>';
    }
}

function renderHistorial(data, dias) {
    const diasConDatos = data.dias.filter(d => d.total > 0);
    if (diasConDatos.length === 0) {
        return '<div class="historial-vacio">Sin sesiones en los últimos ' + dias + ' días</div>';
    }

    // Máximo para escalar las barras
    const maxTotal = Math.max(...data.dias.map(d => d.total));

    const DIAS_ES = ['dom', 'lun', 'mar', 'mié', 'jue', 'vie', 'sáb'];
    const hoy = new Date();
    hoy.setHours(0, 0, 0, 0);
    const ayer = new Date(hoy); ayer.setDate(ayer.getDate() - 1);

    let html = '<div class="historial-dias">';

    data.dias.forEach(dia => {
        if (dia.total === 0) return; // omitir días vacíos

        const fecha = new Date(dia.timestamp * 1000);
        fecha.setHours(0, 0, 0, 0);
        let etiqueta;
        if (fecha.getTime() === hoy.getTime()) {
            etiqueta = 'Hoy';
        } else if (fecha.getTime() === ayer.getTime()) {
            etiqueta = 'Ayer';
        } else {
            etiqueta = DIAS_ES[fecha.getDay()] + ' ' + fecha.getDate() + '/' + (fecha.getMonth() + 1);
        }

        const totalFmt = formatearDuracion(dia.total);
        const anchoPct = maxTotal > 0 ? Math.round((dia.total / maxTotal) * 100) : 0;

        // Segmentos de la barra
        const segmentos = dia.por_actividad.map(act => {
            const pct = dia.total > 0 ? (act.tiempo / dia.total) * 100 : 0;
            return `<div class="historial-seg" style="width:${pct.toFixed(1)}%;background:${act.actividad_color}" title="${act.actividad_nombre}: ${formatearDuracion(act.tiempo)}"></div>`;
        }).join('');

        html += `
            <div class="historial-fila">
                <div class="historial-fecha">${etiqueta}</div>
                <div class="historial-barra-wrap">
                    <div class="historial-barra" style="width:${anchoPct}%">${segmentos}</div>
                </div>
                <div class="historial-total">${totalFmt}</div>
            </div>`;
    });

    html += '</div>';

    // Tabla resumen
    if (data.totales && data.totales.length > 0) {
        html += '<div class="historial-resumen">';
        html += '<div class="historial-resumen-titulo">Total del período</div>';
        data.totales.forEach(act => {
            html += `
                <div class="historial-resumen-fila">
                    <div class="historial-resumen-dot" style="background:${act.actividad_color}"></div>
                    <div class="historial-resumen-nombre">${act.actividad_nombre}</div>
                    <div class="historial-resumen-tiempo">${formatearDuracion(Number(act.tiempo))}</div>
                </div>`;
        });
        const totalPeriodo = data.totales.reduce((s, a) => s + Number(a.tiempo), 0);
        html += `
            <div class="historial-resumen-fila historial-resumen-total">
                <div class="historial-resumen-dot"></div>
                <div class="historial-resumen-nombre">Total</div>
                <div class="historial-resumen-tiempo">${formatearDuracion(totalPeriodo)}</div>
            </div>`;
        html += '</div>';
    }

    return html;
}

// === INICIAR APP AL CARGAR ===

document.addEventListener('DOMContentLoaded', inicializarApp);
