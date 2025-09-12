# MeltForge

**Meltforge** is a universal, plugin-based converter for video, audio, images and more.  
It’s **fast**, **extensible**, and **open source** – built with Rust and Slint.

>  Drop in your media -> Melt it down ->  Forge it into any format you need.

##  Elevator Pitch

Meltforge is the **last converter you’ll ever need**.  
It’s designed to be **lightweight by default** and **extendible on demand**:

- Only install the converters you need (video, audio, image, …).
- Use the **built-in Slint desktop UI** for a friendly drag-and-drop workflow.
- Prefer the terminal? The **CLI** is always there, fast and scriptable.
- Add more power with plugins – Meltforge is designed to grow with you.

**One tool, endless conversions.**

Alright Bro – hier kriegst du die komplette Ladung: **alle User Stories**, schön geordnet nach Priority (P0–P3), jeweils mit **Acceptance Criteria**. Kannst du direkt reviewen, kürzen oder umbauen.

# **Meltforge – User Stories & Priorities**


## **Priority 0 (Core – must exist before anything else)**

### 1. CLI Conversion (Single File)

* **Story:**
  *As a power user, I want to run `meltforge convert input.mp4 --to mp3`, so that I can convert media fast from the terminal.*
* **Acceptance Criteria:**

  * CLI accepts input file and `--to` flag.
  * Conversion runs successfully and outputs target file.
  * Errors (unsupported format, missing file) are clear and exit with non-zero code.

### 2. Core without UI (Lightweight Base)

* **Story:**
  *As a user, I want Meltforge core to run without UI, so that the default install is lightweight.*
* **Acceptance Criteria:**

  * Meltforge builds and runs as CLI only.
  * No Slint/UI dependencies included in minimal build.
  * CLI binary works standalone.

### 3. Plugin Runtime (Discovery & Loading)

* **Story:**
  *As a user, I want Meltforge to auto-load plugins from a `plugins/` folder, so that I only use what’s installed.*
* **Acceptance Criteria:**

  * `plugins/` folder is scanned at startup.
  * Compatible plugins are loaded automatically.
  * If a plugin is broken, Meltforge fails gracefully with a clear error.

## **Priority 1 (Usability & First Extensions)**

### 4. CLI Batch Conversion

* **Story:**
  *As a power user, I want to convert multiple files with wildcards, so that I save time on repetitive tasks.*
* **Acceptance Criteria:**

  * CLI accepts `*.wav` or multiple inputs.
  * Each file is converted in sequence.
  * Errors don’t stop the whole batch (skip failed, log it).

### 5. UI as Optional Package

* **Story:**
  *As a casual user, I want to install a desktop UI only if I need it, so that I don’t pull unnecessary dependencies.*
* **Acceptance Criteria:**

  * UI shipped as a separate binary/package.
  * Meltforge core runs without UI.
  * Installing UI adds drag-and-drop interface.

### 6. Basic UI Drag & Drop

* **Story:**
  *As a casual user, I want to drag & drop files into Meltforge’s UI, so that I can convert easily without CLI.*
* **Acceptance Criteria:**

  * User can drop one or multiple files.
  * Output format can be chosen from a dropdown.
  * Progress bar shown per file.
  * Output saved in same folder by default.

### 7. Basic Settings

* **Story:**
  *As a user, I want to configure default output folder and quality, so that I don’t re-select every time.*
* **Acceptance Criteria:**

  * Config file (`settings.toml` or similar).
  * Default output directory configurable.
  * Quality/bitrate defaults configurable.

## **Priority 2 (Plugins & Developer Focus)**

### 8. Plugin Management CLI

* **Story:**
  *As a user, I want `meltforge plugin install/remove/list`, so that I can manage plugins on demand.*
* **Acceptance Criteria:**

  * CLI command `plugin list` shows installed plugins.
  * `plugin install` downloads from registry or Git URL.
  * `plugin remove` deletes plugin safely.

### 9. Plugin Auto-Discovery in UI/CLI

* **Story:**
  *As a user, I want Meltforge to list available formats from installed plugins, so that I know what’s supported.*
* **Acceptance Criteria:**

  * CLI command `meltforge list-formats` shows all formats.
  * UI dropdown auto-updates when plugin is added/removed.

### 10. Plugin SDK (Stable API)

* **Story:**
  *As a developer, I want a stable Rust SDK, so that I can write plugins without touching core.*
* **Acceptance Criteria:**

  * `meltforge-plugin-sdk` crate published.
  * Clear traits for input/output/metadata.
  * Versioned API to avoid breaking changes.

### 11. Plugin Template & Docs

* **Story:**
  *As a developer, I want a plugin template and example, so that I can scaffold my plugin fast.*
* **Acceptance Criteria:**

  * `cargo generate meltforge-plugin-template`.
  * Example plugin included (e.g., WebP).
  * Docs explaining lifecycle and publishing.

### 12. Plugin Capabilities & Security

* **Story:**
  *As a user, I want plugins to declare capabilities, so that Meltforge can sandbox and prevent misuse.*
* **Acceptance Criteria:**

  * Plugin manifest (Name, Version, Capabilities, Dependencies).
  * Meltforge validates manifest at load.
  * Incompatible plugins rejected cleanly.


## **Priority 3 (Nice-to-Have / Future Ideas)**

### 13. Presets

* **Story:**
  *As a user, I want reusable conversion presets (e.g., “WhatsApp 720p”), so that I don’t configure each time.*
* **Acceptance Criteria:**

  * CLI: `--preset whatsapp`.
  * UI: preset picker dropdown.
  * Presets stored in config (JSON/YAML).

### 14. Queue & Batch Jobs

* **Story:**
  *As a power user, I want a conversion queue, so that I can stack multiple jobs hands-off.*
* **Acceptance Criteria:**

  * CLI: `meltforge queue add input.mp4`.
  * UI: queue list with progress bars.
  * Jobs can be cancelled individually.

### 15. Scripting Configs

* **Story:**
  *As a power user, I want to run conversions from JSON/YAML configs, so that I can automate pipelines.*
* **Acceptance Criteria:**

  * CLI: `meltforge run config.yaml`.
  * Config supports multiple files, options, presets.
  * Errors reported per job.

### 16. Plugin Store/Registry

* **Story:**
  *As a developer, I want a plugin registry, so that the community can share and install plugins easily.*
* **Acceptance Criteria:**

  * Online index of plugins (GitHub registry or custom).
  * `meltforge plugin search <term>`.
  * Secure download + version checks.

