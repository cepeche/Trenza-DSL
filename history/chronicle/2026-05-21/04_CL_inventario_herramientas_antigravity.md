# Inventario de Herramientas — Antigravity (Sesión 2026-05-21)

> **Modelo activo:** Claude Opus 4.6 (Thinking)  
> **Generado:** 2026-05-21T20:08+02:00  
> **Propósito:** Inventario exhaustivo para coordinación con Claude Code en sesión paralela.

---

## 1. Herramientas Nativas (Built-in Tools)

### 1.1 `ask_permission`
- **Parámetros principales:** `Action` (enum: read_url, execute_url, command, unsandboxed, mcp, custom, read_file, write_file), `Target`, `Reason`
- **Descripción:** Solicita permisos adicionales al usuario cuando una operación falla por permisos insuficientes (lectura/escritura de archivos, ejecución de comandos, etc.).

### 1.2 `ask_question`
- **Parámetros principales:** `questions` (array de objetos con `question`, `options`, `is_multi_select`)
- **Descripción:** Presenta preguntas de opción múltiple al usuario en un modal interactivo para clarificar requisitos, solicitar preferencias o resolver ambigüedades.

### 1.3 `define_subagent`
- **Parámetros principales:** `name`, `description`, `system_prompt`, `enable_write_tools`, `enable_mcp_tools`, `enable_subagent_tools`
- **Descripción:** Define un nuevo tipo de subagente especializado con su propio prompt y conjunto de herramientas, invocable posteriormente con `invoke_subagent`.

### 1.4 `generate_image`
- **Parámetros principales:** `Prompt`, `ImageName`, `ImagePaths` (opcional, hasta 3 imágenes para editar/combinar)
- **Descripción:** Genera o edita imágenes a partir de un prompt de texto. Útil para mockups de UI, assets de aplicaciones web, o iteración de diseño visual.

### 1.5 `grep_search`
- **Parámetros principales:** `SearchPath`, `Query`, `Includes` (globs), `IsRegex`, `CaseInsensitive`, `MatchPerLine`
- **Descripción:** Busca patrones exactos o regex dentro de archivos/directorios usando ripgrep. Devuelve resultados en JSON (máx. 50 coincidencias).

### 1.6 `invoke_subagent`
- **Parámetros principales:** `Subagents` (array con `TypeName`, `Role`, `Prompt`, `Workspace`)
- **Descripción:** Lanza uno o más subagentes en segundo plano. Cada uno ejecuta con su propio prompt y devuelve resultados cuando termina. Soporta workspaces heredados, ramificados o compartidos.

### 1.7 `list_dir`
- **Parámetros principales:** `DirectoryPath`
- **Descripción:** Lista el contenido de un directorio (archivos, subdirectorios, tamaños, conteo recursivo de hijos).

### 1.8 `list_permissions`
- **Parámetros principales:** *(ninguno funcional)*
- **Descripción:** Muestra todos los permisos actualmente concedidos en la sesión.

### 1.9 `manage_subagents`
- **Parámetros principales:** `Action` (enum: list, kill, kill_all), `ConversationIds` (para kill)
- **Descripción:** Gestiona subagentes activos: listar, matar específicos, o matar todos (incluidos descendientes).

### 1.10 `manage_task`
- **Parámetros principales:** `Action` (enum: list, kill, status, send_input), `TaskId`, `Input`
- **Descripción:** Gestiona tareas en segundo plano: listar, cancelar, comprobar estado, o enviar input stdin a una tarea en ejecución.

### 1.11 `multi_replace_file_content`
- **Parámetros principales:** `TargetFile`, `Instruction`, `Description`, `ReplacementChunks` (array con `StartLine`, `EndLine`, `TargetContent`, `ReplacementContent`, `AllowMultiple`), `ArtifactMetadata`
- **Descripción:** Edita un archivo existente realizando **múltiples reemplazos no contiguos** en una sola llamada. Cada chunk especifica el bloque exacto a reemplazar.

### 1.12 `read_url_content`
- **Parámetros principales:** `Url`
- **Descripción:** Obtiene contenido de una URL vía HTTP, convirtiendo HTML a markdown. Sin ejecución de JavaScript ni autenticación.

### 1.13 `replace_file_content`
- **Parámetros principales:** `TargetFile`, `StartLine`, `EndLine`, `TargetContent`, `ReplacementContent`, `AllowMultiple`, `Instruction`, `Description`
- **Descripción:** Edita un archivo existente realizando **un único reemplazo contiguo**. Para múltiples ediciones no adyacentes, usar `multi_replace_file_content`.

### 1.14 `run_command`
- **Parámetros principales:** `CommandLine`, `Cwd`, `WaitMsBeforeAsync`
- **Descripción:** Propone y ejecuta un comando en PowerShell (Windows). El usuario debe aprobar antes de la ejecución. Soporta ejecución síncrona y en segundo plano.

### 1.15 `schedule`
- **Parámetros principales:** `DurationSeconds` (one-shot) | `CronExpression` (recurrente), `Prompt`, `MaxIterations`
- **Descripción:** Programa un temporizador de una sola vez o un cron job recurrente que envía notificaciones en segundo plano.

### 1.16 `search_web`
- **Parámetros principales:** `query`, `domain` (opcional)
- **Descripción:** Realiza búsquedas web y devuelve un resumen con citas de URLs relevantes.

### 1.17 `send_message`
- **Parámetros principales:** `Recipient` (conversation ID), `Message`
- **Descripción:** Envía un mensaje a otro agente (subagente o peer). **No** para comunicarse con el usuario humano.

### 1.18 `view_file`
- **Parámetros principales:** `AbsolutePath`, `StartLine`, `EndLine`, `IsSkillFile`
- **Descripción:** Visualiza el contenido de un archivo (texto o binario: imagen, vídeo). Máximo 800 líneas por lectura.

### 1.19 `write_to_file`
- **Parámetros principales:** `TargetFile`, `CodeContent`, `Overwrite`, `Description`, `IsArtifact`, `ArtifactMetadata`
- **Descripción:** Crea un nuevo archivo (con directorios padres si es necesario). Puede crear artefactos con metadatos. Opcionalmente sobrescribe archivos existentes.

---

## 2. Subagentes Pre-definidos

### 2.1 `research`
- **Descripción:** Subagente de solo lectura para explorar el codebase, buscar en la web y leer archivos. Se delega cuando la investigación requiere muchos pasos de búsqueda.

### 2.2 `self`
- **Descripción:** Subagente que hereda la configuración completa del agente padre (herramientas, prompt, modelo). Para ejecutar tareas en un contexto de conversación separado con las mismas capacidades.

---

## 3. Skills Disponibles (Plugins)

### 3.1 Ciencia / Bioinformática (`science` plugin)

| Skill | Descripción breve |
|---|---|
| `alphafold-database-fetch-and-analyze` | Recupera y analiza estructuras predichas por AlphaFold dado un UniProt ID (pLDDT, dominios, desorden). |
| `alphagenome-single-variant-analysis` | Analiza efectos de variantes genéticas no codificantes sobre expresión, cromatina, histonas y TFs vía AlphaGenome API. |
| `chembl-database` | Consulta ChEMBL para moléculas bioactivas, targets, IC50/Ki, fármacos aprobados y estructuras químicas. |
| `clinical-trials-database` | Consulta ClinicalTrials.gov APIv2: búsqueda por condición, fármaco, localización, fase, estado, NCT ID. |
| `clinvar-database` | Clasificaciones de patogenicidad (Pathogenic, Benign, VUS), significancia clínica y controles benchmark para variantes humanas. |
| `dbsnp-database` | Busca/mapea SNPs e indels en dbSNP: rsIDs ↔ coordenadas genómicas ↔ HGVS, frecuencias alélicas. |
| `embl-ebi-ols` | Consulta ontologías biomédicas (GO, DOID, HP, etc.) en OLS: términos, jerarquías, propiedades, autocompletado. |
| `encode-ccres-database` | Consulta el registro ENCODE de elementos cis-reguladores (cCREs) vía GraphQL y datos experimentales (ChIP-seq). |
| `ensembl-database` | Resuelve IDs de genes/transcritos/proteínas, obtiene secuencias genómicas, estructuras de exones y predicciones VEP. |
| `foldseek-structural-search` | Búsqueda 3D estructural contra PDB, AlphaFold, CATH, etc. Requiere un archivo .cif/.pdb físico. |
| `gnomad-database` | Frecuencias alélicas, métricas de constraint (pLI, LOEUF), variantes por región/gen en gnomAD. |
| `gtex-database` | Datos cuantitativos de expresión RNA y eQTLs del proyecto GTEx (54 tejidos). |
| `human-protein-atlas-database` | Expresión proteica semi-cuantitativa y localización espacial del Human Protein Atlas. |
| `interpro-database` | Identifica dominios, familias y sitios en proteínas. Combina 14 bases de datos (Pfam, CDD, etc.). Búsqueda IDA. |
| `jaspar-database` | Perfiles de unión de factores de transcripción: PFM/PWM, matrix IDs, formatos MEME/TRANSFAC. |
| `literature-search-arxiv` | Busca papers en arXiv: metadatos, abstracts, descarga de PDFs/HTML. |
| `literature-search-biorxiv` | Navega preprints de bioRxiv/medRxiv por fecha, categoría y palabras clave. |
| `literature-search-europepmc` | Busca en Europe PMC: full-text XML, citas, bibliografía, PDFs open-access. |
| `literature-search-openalex` | Consulta OpenAlex: papers, autores, instituciones, tópicos, bibliometría, DOIs, PDFs open-access. |
| `ncbi-sequence-fetch` | Recupera secuencias de proteínas y nucleótidos de NCBI por accession, gen, locus, PubMed ID o patente. |
| `openfda-database` | Consulta openFDA: eventos adversos, recalls, etiquetado, aprobaciones, escasez, NDC, 28 endpoints. |
| `opentargets-database` | Asociaciones target-enfermedad, descubrimiento de dianas, tractabilidad, seguridad, fármacos conocidos. |
| `pdb-database` | Busca/descarga estructuras 3D experimentales de biomoléculas en PDB (similitud de secuencia/estructura). |
| `protein-sequence-msa` | Alineamiento múltiple de secuencias proteicas con Clustal Omega (hasta 4000 secuencias). |
| `protein-sequence-similarity-search` | Busca homólogos proteicos vía MMseqs2 (rápido) o BLAST (comprehensivo). |
| `pubchem-database` | Consulta PubChem: búsqueda por nombre/CID/SMILES, propiedades, similitud, bioactividad. |
| `pubmed-database` | Busca literatura en PubMed, abstracts, full-text, enlaces a bases de datos biológicas. |
| `pymol` | Visualización, análisis y renderizado de estructuras moleculares con PyMOL. |
| `quickgo-database` | Mapeo genes ↔ GO terms, jerarquía de Gene Ontology, ECO. |
| `reactome-database` | Análisis de pathways, enrichment de listas de genes, jerarquía de pathways, diagramas. |
| `science-skills-common` | Paquete compartido (http_client con rate limiting, retries). No invocar directamente. |
| `string-database` | Interacciones proteína-proteína (PPIs), enrichment funcional, homología en STRING. |
| `ucsc-conservation-and-tfbs` | Scores de conservación (phyloP, phastCons) y TFBS del UCSC Genome Browser. |
| `unibind-database` | Sitios de unión TF experimentalmente validados: datasets por especie, línea celular o TF. |
| `uniprot-database` | Metadatos de proteínas, función, taxonomía, secuencias en UniProtKB/UniParc/UniRef. |
| `uv` | Verifica/instala el gestor de paquetes Python `uv`. Prerrequisito para otros skills. |
| `workflow-skill-creator` | Destila un workflow completado en un skill reutilizable. |

### 3.2 Android (`android-cli-plugin`)

| Skill | Descripción breve |
|---|---|
| `android-cli` | Orquesta tareas de desarrollo Android: creación de proyecto, despliegue, gestión de SDK, diagnóstico de entorno. |

### 3.3 Otros Plugins (sin skills directos)

| Plugin | Descripción breve |
|---|---|
| `chrome-devtools-plugin` | Plugin para DevTools de Chrome (configuración, sin skills expuestos). |
| `modern-web-guidance-plugin` | Guías de desarrollo web moderno (configuración, sin skills expuestos). |

---

## 4. Slash Commands (recomendables al usuario)

| Comando | Descripción |
|---|---|
| `/goal` | Tarea de larga duración — el agente no para hasta completar el objetivo. |
| `/schedule` | Ejecutar instrucciones en un horario recurrente o temporizador único. |
| `/browser` | Tareas que implican navegación web o interacción con aplicaciones web. |
| `/grill-me` | Entrevista interactiva para alinear un plan resolviendo decisiones de diseño. |

---

## 5. Resumen Cuantitativo

| Categoría | Cantidad |
|---|---|
| Herramientas nativas (tools) | **19** |
| Subagentes pre-definidos | **2** |
| Skills científicos | **33** |
| Skills Android | **1** |
| Plugins (total) | **4** |
| Slash commands | **4** |
| **Total capacidades enumeradas** | **63** |

---

*Inventario generado por Antigravity (Claude Opus 4.6 Thinking) para coordinación inter-sesión.*
