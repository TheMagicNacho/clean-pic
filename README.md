# clean-pic

A Tauri application for cleaning pictures, built with TypeScript and Svelte.

## Tech Stack

- **Tauri 2** - Rust-based desktop application framework
- **SvelteKit** - Frontend framework with TypeScript
- **Vite** - Build tool and development server

## Supported Platforms

- Linux
- macOS

## Prerequisites

Before you begin, ensure you have the following installed:

1. **Node.js** (v20 or later) and npm
2. **Rust** (latest stable)
3. **System dependencies** for your platform:
   - **Linux**: webkit2gtk, gtk3, libayatana-appindicator3, librsvg2
     ```bash
     # Debian/Ubuntu
     sudo apt update
     sudo apt install libwebkit2gtk-4.1-dev \
       build-essential \
       curl \
       wget \
       file \
       libssl-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev
     ```
   - **macOS**: Xcode Command Line Tools
     ```bash
     xcode-select --install
     ```

For more details, visit the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/).

## Getting Started

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Run in development mode:**
   ```bash
   npm run tauri dev
   ```

3. **Build for production:**
   ```bash
   npm run tauri build
   ```

## Development

- **TypeScript checking:** `npm run check`
- **Watch mode for type checking:** `npm run check:watch`

## Project Structure

- `/src` - Svelte/TypeScript frontend code
- `/src-tauri` - Rust backend code
- `/static` - Static assets

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) with:
- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
