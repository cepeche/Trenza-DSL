# Licencias de componentes de terceros

Mi Cronómetro PSP es software de uso personal. Los componentes de terceros que utiliza son:

---

## Runtime y servidor

### PHP 8.x
- **Uso**: Lenguaje del backend (API REST)
- **Licencia**: PHP License v3.01
- **URL**: https://www.php.net/license/3_01.txt

### Apache HTTP Server
- **Uso**: Servidor web embebido en el NAS WD My Cloud EX2 Ultra
- **Licencia**: Apache License 2.0
- **URL**: https://www.apache.org/licenses/LICENSE-2.0

### SQLite
- **Uso**: Base de datos embebida (cronometro.db)
- **Licencia**: Dominio público (Public Domain)
- **URL**: https://www.sqlite.org/copyright.html

---

## Frontend

### Material Symbols (Google Fonts)
- **Uso**: Iconos de la interfaz (play, stop, edit, etc.)
- **Licencia**: Apache License 2.0
- **URL**: https://github.com/google/material-design-icons/blob/master/LICENSE
- **Carga**: CDN de Google Fonts (`fonts.googleapis.com`)

---

## Herramientas de desarrollo y test

### PHPUnit 11
- **Uso**: Tests unitarios de PHP (`tests/php/`)
- **Versión**: 11.5.55 (phpunit.phar)
- **Licencia**: BSD 3-Clause License
- **URL**: https://github.com/sebastianbergmann/phpunit/blob/main/LICENSE
- **Nota**: Solo se usa en desarrollo/CI, no se despliega en el NAS.

### Node.js
- **Uso**: Ejecutar los tests de JavaScript (`tests/js/runner.js`)
- **Licencia**: MIT License (con dependencias bajo varias licencias permisivas)
- **URL**: https://github.com/nodejs/node/blob/main/LICENSE
- **Nota**: Solo se usa en desarrollo/CI.

---

## Nota sobre el código propio

Todo el código fuente del proyecto (frontend, backend PHP, scripts de deploy y
scripts de test) es de autoría propia y no tiene licencia de distribución
pública asociada.
