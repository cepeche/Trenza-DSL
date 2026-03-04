# Mi Cronómetro PSP - Estructura del Proyecto

**Versión**: 1.0  
**Fecha**: 17 febrero 2026

---

## Estructura de Carpetas

```
mi-cronometro-psp/
├── docs/                           # Documentación del proyecto
│   ├── README.md                   # Documentación principal (copiado aquí también)
│   ├── requisitos.md               # Especificación de requisitos funcionales
│   ├── plan-pruebas.md             # Estrategia y casos de prueba
│   ├── arquitectura-time-tracker.md # Diseño arquitectónico
│   ├── futuras-versiones.md        # Roadmap y features futuras
│   └── configuracion-red-local.md  # Guía de despliegue en NAS
│
├── src/                            # Código fuente (a desarrollar)
│   ├── api/                        # Backend PHP
│   │   ├── index.php
│   │   ├── endpoints/
│   │   ├── models/
│   │   └── db/
│   │
│   ├── frontend/                   # Frontend web
│   │   ├── index.html
│   │   ├── css/
│   │   ├── js/
│   │   └── assets/
│   │
│   └── database/                   # Scripts SQL
│       ├── schema.sql
│       └── seed.sql
│
├── scripts/                        # Scripts de utilidad (a crear)
│   ├── backup.sh
│   ├── deploy.sh
│   └── reset-db.sh
│
└── tests/                          # Tests (a implementar)
    ├── unit/
    ├── integration/
    └── e2e/
```

---

## Primeros Pasos

### 1. Lee la Documentación

Empieza por estos documentos en orden:

1. **docs/README.md** - Visión general del proyecto
2. **docs/requisitos.md** - Qué debe hacer la aplicación
3. **docs/arquitectura-time-tracker.md** - Cómo está diseñada
4. **docs/configuracion-red-local.md** - Cómo desplegarla en tu NAS

### 2. Prepara tu Entorno

Según **configuracion-red-local.md**:
- Conecta al NAS por SSH (192.168.1.71)
- Verifica servicios instalados
- Crea estructura de directorios en el NAS

### 3. Desarrolla con Claude Code

Abre esta carpeta en **Claude Code** para:
- Implementar backend PHP
- Crear frontend JavaScript
- Escribir tests
- Desplegar en el NAS

---

## Estado Actual del Proyecto

- ✅ **Documentación completa**
- ✅ **Requisitos especificados**
- ✅ **Arquitectura definida**
- ✅ **Plan de pruebas preparado**
- ⏳ **Implementación pendiente**

---

## Próximos Pasos

1. Abrir proyecto en **Claude Code**
2. Conectar al NAS (192.168.1.71)
3. Configurar servidor web
4. Crear base de datos SQLite
5. Implementar backend PHP
6. Desarrollar frontend
7. Testing en red local

---

## Soporte

Para dudas sobre el proyecto, consulta:
- **docs/requisitos.md** - Especificaciones funcionales
- **docs/plan-pruebas.md** - Estrategia de testing
- **docs/configuracion-red-local.md** - Deployment y troubleshooting

---

**¡Buena suerte con el desarrollo!** 🚀
