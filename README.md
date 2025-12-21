# Tauri Plugin Router

A Tauri plugin that provides a custom URI-based command routing system for Tauri applications. Instead of using Tauri's built-in IPC, this plugin enables HTTP-style routing over a custom protocol handler.

## TODOs:

- Fix `Result` commands serialized to `{ "Ok": T }` or `{ "Err": E}`
- Middleware/Layers

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-router = { git = "https://github.com/your-username/tauri-plugin-router" }
```

### JavaScript/TypeScript

```bash
npm install tauri-plugin-router
# or
pnpm add tauri-plugin-router
# or
yarn add tauri-plugin-router
```

## Usage

Create a `Router`, add your commandd and initialize the plugin

`src-tauri/src/lib.rs`:

```rs
use tauri_plugin_router::Router;

fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let router = Router::new().command("greet", greet);

    tauri::Builder::default()
        .plugin(tauri_plugin_router::init(router))
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

then call your command from the frontend:

```ts
import { invoke } from "tauri-plugin-router";

const result = await invoke("greet", "Amr");
console.log(result); // Hello Amr, You've been greeted from Rust!
```

## License

MIT or Apache-2.0
