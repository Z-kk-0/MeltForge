# MeltForge
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/Z-kk-0/MeltForge?utm_source=oss&utm_medium=github&utm_campaign=Z-kk-0%2FMeltForge&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

**Meltforge** is a universal, plugin-based converter for video, audio, images and more.  
It’s **fast**, **extensible**, and **open source** – built with Rust and Slint.

> Drop in your media -> Melt it down -> Forge it into any format you need.

## Elevator Pitch

Meltforge is the **last converter you’ll ever need**.  
It’s designed to be **lightweight by default** and **extendible on demand**:

- Only install the converters you need (video, audio, image, …).
- Use the **built-in Slint desktop UI** for a friendly drag-and-drop workflow.
- Prefer the terminal? The **CLI** is always there, fast and scriptable.
- Add more power with plugins – Meltforge is designed to grow with you.

**One tool, endless conversions.**

# **Meltforge – User Stories & Priorities**

## **Priority 0 (Core – must exist before anything else)**

### 1. CLI Conversion (Single File)

- **Story:**
  _As a power user, I want to run `meltforge convert input.mp4 --to mp3`, so that I can convert media fast from the terminal._
- **Acceptance Criteria:**

  - CLI accepts input file and `--to` flag.
  - Conversion runs successfully and outputs target file.
  - Errors (unsupported format, missing file) are clear and exit with non-zero code.

### 2. Core without UI (Lightweight Base)

- **Story:**
  _As a user, I want Meltforge core to run without UI, so that the default install is lightweight._
- **Acceptance Criteria:**

  - Meltforge builds and runs as CLI only.
  - No Slint/UI dependencies included in minimal build.
  - CLI binary works standalone.

### 3. Plugin Runtime (Discovery & Loading)

- **Story:**
  _As a user, I want Meltforge to auto-load plugins from a `plugins/` folder, so that I only use what’s installed._
- **Acceptance Criteria:**

  - `plugins/` folder is scanned at startup.
  - Compatible plugins are loaded automatically.
  - If a plugin is broken, Meltforge fails gracefully with a clear error.

## **Priority 1 (Usability & First Extensions)**

### 4. CLI Batch Conversion

- **Story:**
  _As a power user, I want to convert multiple files with wildcards, so that I save time on repetitive tasks._
- **Acceptance Criteria:**

  - CLI accepts `*.wav` or multiple inputs.
  - Each file is converted in sequence.
  - Errors don’t stop the whole batch (skip failed, log it).

### 5. UI as Optional Package

- **Story:**
  _As a casual user, I want to install a desktop UI only if I need it, so that I don’t pull unnecessary dependencies._
- **Acceptance Criteria:**

  - UI shipped as a separate binary/package.
  - Meltforge core runs without UI.
  - Installing UI adds drag-and-drop interface.

### 6. Basic UI Drag & Drop

- **Story:**
  _As a casual user, I want to drag & drop files into Meltforge’s UI, so that I can convert easily without CLI._
- **Acceptance Criteria:**

  - User can drop one or multiple files.
  - Output format can be chosen from a dropdown.
  - Progress bar shown per file.
  - Output saved in same folder by default.

### 7. Basic Settings

- **Story:**
  _As a user, I want to configure default output folder and quality, so that I don’t re-select every time._
- **Acceptance Criteria:**

  - Config file (`settings.toml` or similar).
  - Default output directory configurable.
  - Quality/bitrate defaults configurable.

## **Priority 2 (Plugins & Developer Focus)**

### 8. Plugin Management CLI

- **Story:**
  _As a user, I want `meltforge plugin install/remove/list`, so that I can manage plugins on demand._
- **Acceptance Criteria:**

  - CLI command `plugin list` shows installed plugins.
  - `plugin install` downloads from registry or Git URL.
  - `plugin remove` deletes plugin safely.

### 9. Plugin Auto-Discovery in UI/CLI

- **Story:**
  _As a user, I want Meltforge to list available formats from installed plugins, so that I know what’s supported._
- **Acceptance Criteria:**

  - CLI command `meltforge list-formats` shows all formats.
  - UI dropdown auto-updates when plugin is added/removed.

### 10. Plugin SDK (Stable API)

- **Story:**
  _As a developer, I want a stable Rust SDK, so that I can write plugins without touching core._
- **Acceptance Criteria:**

  - `meltforge-plugin-sdk` crate published.
  - Clear traits for input/output/metadata.
  - Versioned API to avoid breaking changes.

### 11. Plugin Template & Docs

- **Story:**
  _As a developer, I want a plugin template and example, so that I can scaffold my plugin fast._
- **Acceptance Criteria:**

  - `cargo generate meltforge-plugin-template`.
  - Example plugin included (e.g., WebP).
  - Docs explaining lifecycle and publishing.

### 12. Plugin Capabilities & Security

- **Story:**
  _As a user, I want plugins to declare capabilities, so that Meltforge can sandbox and prevent misuse._
- **Acceptance Criteria:**

  - Plugin manifest (Name, Version, Capabilities, Dependencies).
  - Meltforge validates manifest at load.
  - Incompatible plugins rejected cleanly.

## **Priority 3 (Nice-to-Have / Future Ideas)**

### 13. Presets

- **Story:**
  _As a user, I want reusable conversion presets (e.g., “WhatsApp 720p”), so that I don’t configure each time._
- **Acceptance Criteria:**

  - CLI: `--preset whatsapp`.
  - UI: preset picker dropdown.
  - Presets stored in config (JSON/YAML).

### 14. Queue & Batch Jobs

- **Story:**
  _As a power user, I want a conversion queue, so that I can stack multiple jobs hands-off._
- **Acceptance Criteria:**

  - CLI: `meltforge queue add input.mp4`.
  - UI: queue list with progress bars.
  - Jobs can be cancelled individually.

### 15. Scripting Configs

- **Story:**
  _As a power user, I want to run conversions from JSON/YAML configs, so that I can automate pipelines._
- **Acceptance Criteria:**

  - CLI: `meltforge run config.yaml`.
  - Config supports multiple files, options, presets.
  - Errors reported per job.

### 16. Plugin Store/Registry

- **Story:**
  _As a developer, I want a plugin registry, so that the community can share and install plugins easily._
- **Acceptance Criteria:**

  - Online index of plugins (GitHub registry or custom).
  - `meltforge plugin search <term>`.
  - Secure download + version checks.

# Planning

## #1 User Story – CLI Conversion

For the first user story I will set up a Cargo workspace with two crates:

1. **`mf-core`** – a lightweight library that defines the conversion skeleton, core types, validation logic, and the conversion pipeline. Other MeltForge components such as `mf-cli` and future plugins will depend on this library.
2. **`mf-cli`** – the command-line interface for MeltForge, acting as the initial user-facing entry point.

To verify the setup, a basic conversion from PNG to JPEG will be implemented. This is only a proof of functionality and will later be extracted into a standalone plugin.

### mf-core

The conversion pipeline (for now) follows this sequence:

**Command → Validation → Conversion → Output → Response**

A conversion job is submitted to `mf-core`. The core validates the request (e.g., input file exists, target format is supported). If validation succeeds, the conversion is executed — initially through a built-in routine, and later via the appropriate plugin. The result is then returned as either:

- **Success** – including the output path.
- **Error** – with a descriptive message.

### mf-cli

The CLI will be released together with the core as the default interface. Later, a Slint-based desktop UI will be available as an optional component.

The CLI is aimed at users familiar with terminal tools. Its usage looks like this:

```bash
meltforge convert file.jpg --to png
```

Here `--to` specifies the target format.

## #4 UserStory 

First i have to make it technically possible to convert multiple files in the core itsef because currently it only has

```rust
pub fn convert(cj: ConvertJob) -> Result<PathBuf, MeltforgeError> {
    validate_job(&cj)?; // Validate

    let mut output_path = cj
        .output
        .clone()
        .unwrap_or_else(|| derive_output_path(&cj.input, cj.format_type));

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| map_io_write(e, parent.to_path_buf()))?;
        }
    }
    let input_fmt = detect_input_format(&cj.input).map_err(|e| MeltforgeError::from(e))?;
    match (input_fmt, cj.format_type) {
        (FormatType::PNG, FormatType::JPEG) => convert_png_jpg(&cj.input, &output_path)?,
        (FormatType::JPEG, FormatType::PNG) => convert_jpg_png(&cj.input, &output_path)?,
        _ => {
            return Err(FormatError::UnsupportedOutput(format!(
                "{:?} → {:?} not supported yet",
                input_fmt, cj.format_type
            ))
            .into());
        }
    } // Convert currently only png  to jpg will later be replaced with the plugin function
```
so basically, a function that takes a list of ConvertJobs and converts them each step by step. Maybe it is possible to multithread this so it converts faster
then the cli will need the capabilities to accept and read wildcards correctly. This will all be in the CLI itself and not in the core so we can seperate concerns. The core should still be flexible and as lightweight as possible.


## #5 User Story
The most important thing would be a area where you can dnd files into and where you can click on the area where you can select your files via the Users desired file manager. If you drop it in you also need an option to set the conversion output and the output path.

### Ui Coloring
its about time for the coloring aspect

background: Charcoal #36454f
primary:  Orange: #F66907
secondary: lava: #cf1020

#### Neutrals

| Role                         | Color     | Use                                                    |
| ---------------------------- | --------- | ------------------------------------------------------ |
| Background (lighter section) | `#2C353C` | Sidebars, modals, subtle contrast from main background |
| Surface / Card               | `#404E57` | Panel backgrounds, hover states                        |
| Divider / Border             | `#5A6A74` | Lines, outlines, separators                            |
| Text Primary                 | `#EAEAEA` | Main text on dark backgrounds                          |
| Text Secondary               | `#B8C2C8` | Captions, labels, disabled text                        |

#### Accent and Utility Colors

| Purpose | Color                | Description                                    |
| ------- | -------------------- | ---------------------------------------------- |
| Success | `#27AE60`            | Subtle green to pop against dark UI            |
| Warning | `#F2C037`            | Warm gold fits naturally beside the orange     |
| Info    | `#3498DB`            | Cooler tone for neutral hints, tooltips, links |
| Error   | use `#CF1020` (lava) | Already perfect – intense, decisive red        |

#### Highlight and Depth

| Variant        | Color     | Use                                 |
| -------------- | --------- | ----------------------------------- |
| Orange Light   | `#FF8C33` | Hover or focus of primary buttons   |
| Orange Dark    | `#C35605` | Active or pressed states            |
| Lava Dark      | `#A00E1A` | Hover state for destructive actions |
| Charcoal Light | `#4A5A64` | Hover for dark panels               |
