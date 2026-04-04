# Copilot Instructions

## Project Overview

**0w0 Scrubby Buddy** is a Tauri 2 desktop app that removes EXIF/metadata from JPEG images. It has two modes: **Scrub** (batch-process a folder) and **Inspect** (view metadata in a single image). The UI features a kawaii bear mascot that reacts to user actions.

## Commands

```bash
# Full app with hot reload (use this for development)
npm run tauri dev        # or: tauri dev

# Frontend only
npm run dev              # Vite dev server on port 1420
npm run check            # Type-check frontend (svelte-check)
npm run check:watch      # Type-check in watch mode

# Production
tauri build              # Runs `npm run build` then packages the app

# Rust only (from src-tauri/)
cargo check              # Check Rust for errors
cargo clippy             # Rust linter
```

There are no automated tests. `npm run check` / `svelte-check` is the primary validation tool for the frontend.

## Architecture

The app is split across a Svelte frontend and a Rust backend connected via Tauri's IPC bridge.

**Frontend (`src/`)** — SvelteKit in SPA mode (SSR disabled, static adapter with `fallback: "index.html"`). There is only one route (`src/routes/+page.svelte`) — no routing library. Manual view switching uses `$state()` + `{#if activeView === "scrub"}`.

**Backend (`src-tauri/src/lib.rs`)** — All image processing logic lives here. Three Tauri commands handle: counting images, scrubbing metadata, and inspecting metadata. File I/O uses Tokio async.

**IPC Pattern** — Frontend calls Rust via `invoke()`, Rust exposes functions with `#[tauri::command]`:
```ts
// Frontend
const result = await invoke("scrub_images", { path: folderPath, saveDirectory });
```
```rust
// Backend (lib.rs)
#[tauri::command]
async fn scrub_images(path: &str, save_directory: &str) -> Result<String, ()> { ... }
```
New commands must be registered in the `.invoke_handler()` call inside `run()` in `lib.rs`.

## Key Conventions

### Svelte 5 Runes
All state uses Svelte 5 runes — not Svelte stores or reactive declarations:
```ts
let folderPath = $state("");
let bearState: BearState = $state("waiting");
```
Component props use `$props()`:
```ts
let { state, message }: Props = $props();
```

### CSS
- No CSS framework — pure scoped CSS inside `<style>` blocks
- Kawaii color palette defined as CSS variables in `+page.svelte` (e.g. `--kawaii-hot-pink: #ff1493`, `--kawaii-soft-pink: #ffb6c1`, `--kawaii-text-dark: #4a0e4e`) — use these for any new UI
- Font is Nunito, loaded via `@font-face` from `/static/fonts/`
- Animations use named `@keyframes`; existing ones include `float`, `bounce`, `tilt`, `sparkle`, `blink`

### Rust
- Async throughout — use `tokio::fs` for file I/O, not `std::fs`
- JPEG segment filtering uses `img-parts`; EXIF parsing uses `kamadak-exif`
- Only JPEG is supported (PNG/TIFF paths are stubbed/commented out)
- Cleaned images are saved with random filenames (using `rand`)

### Tauri Plugins
- `tauri-plugin-dialog` — native file/folder picker dialogs
- `tauri-plugin-opener` — open a folder in the system file explorer
- Both are initialized in `lib.rs` via `.plugin(...)` on the builder
