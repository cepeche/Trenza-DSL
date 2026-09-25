/**
 * Cliente para consumir la API del backend
 * Maneja todas las comunicaciones con el servidor
 */

// Configuración de la API
// - Acceso local (HTTP :8080):   http://192.168.1.71:8080/apps/cronometro
// - Acceso externo (HTTPS :443): https://cronometro.hash-pointer.com/apps/cronometro
// En ambos casos la ruta de API es la misma: .../apps/cronometro/api/...
const API_CONFIG = {
    baseURL: (function() {
        const h = window.location.hostname;
        const proto = window.location.protocol;
        if (proto === 'https:') {
            // Acceso externo: misma ruta pero via HTTPS sin puerto especial
            return `https://${h}/apps/cronometro`;
        } else if (h === 'localhost' || h === '127.0.0.1') {
            return 'http://localhost:8080/apps/cronometro';
        } else {
            // Red local via HTTP :8080
            return `http://${h}:8080/apps/cronometro`;
        }
    })(),
    timeout: 10000
};

class APIClient {
    constructor(baseURL) {
        this.baseURL = baseURL;
    }

    /**
     * Realizar petición HTTP
     */
    async request(endpoint, options = {}) {
        const url = `${this.baseURL}/${endpoint}`;
        const config = {
            method: options.method || 'GET',
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            }
        };

        if (options.body) {
            config.body = JSON.stringify(options.body);
        }

        try {
            const response = await fetch(url, config);
            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.error || `HTTP ${response.status}`);
            }

            return data;
        } catch (error) {
            console.error('Error en petición API:', error);
            throw error;
        }
    }

    // === ACTIVIDADES ===

    async getActividades() {
        return this.request('api/actividades');
    }

    async crearActividad(nombre, color, permanente = false) {
        return this.request('api/actividades', {
            method: 'POST',
            body: { nombre, color, permanente }
        });
    }

    async actualizarActividad(id, datos) {
        return this.request(`api/actividades?id=${id}`, {
            method: 'PUT',
            body: datos
        });
    }

    async archivarActividad(id) {
        return this.request(`api/actividades?id=${id}`, {
            method: 'DELETE'
        });
    }

    // === TIPOS DE TAREA ===

    async getTiposTarea() {
        return this.request('api/tipos-tarea');
    }

    async crearTipoTarea(nombre, icono, actividadesPermitidas) {
        return this.request('api/tipos-tarea', {
            method: 'POST',
            body: {
                nombre,
                icono,
                actividades_permitidas: actividadesPermitidas
            }
        });
    }

    async editarTipoTarea(id, nombre, icono) {
        return this.request('api/tipos-tarea', {
            method: 'PATCH',
            body: { id, nombre, icono }
        });
    }

    // === SESIONES ===

    async getSesiones(fecha = 'hoy') {
        return this.request(`api/sesiones?fecha=${fecha}`);
    }

    async getSesionActiva() {
        return this.request('api/sesiones?action=activa');
    }

    async getEstadisticas(fecha = 'hoy') {
        return this.request(`api/sesiones?action=estadisticas&fecha=${fecha}`);
    }

    async iniciarSesion(tareaId, notas = null, minutosRetroactivos = 0, sustituir = false) {
        return this.request('api/sesiones?action=iniciar', {
            method: 'POST',
            body: { tarea_id: tareaId, notas, minutos_retroactivos: minutosRetroactivos, sustituir }
        });
    }

    async getAcumulado() {
        return this.request('api/sesiones?action=acumulado');
    }

    async getHistorial(dias = 7) {
        return this.request(`api/sesiones?action=historial&dias=${dias}`);
    }

    async resetear(conservar = []) {
        // Devuelve un Blob (CSV), no JSON — necesita fetch directo
        const url = `${this.baseURL}/api/sesiones?action=reset`;
        const response = await fetch(url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ conservar })
        });
        if (!response.ok) {
            const err = await response.json();
            throw new Error(err.error || `HTTP ${response.status}`);
        }
        const blob = await response.blob();
        const filename = (response.headers.get('Content-Disposition') || '')
            .match(/filename="?([^"]+)"?/)?.[1] || 'cronometro-reset.csv';
        return { blob, filename };
    }

    async detenerSesion() {
        return this.request('api/sesiones?action=detener', {
            method: 'POST'
        });
    }

    // === HEALTH CHECK ===

    async healthCheck() {
        return this.request('api/health');
    }
}

// Instancia global del cliente API
const api = new APIClient(API_CONFIG.baseURL);
