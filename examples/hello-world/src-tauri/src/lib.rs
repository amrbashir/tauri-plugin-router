// Sync handler example
fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Async handler example - simulates a database query or API call
async fn async_greet(name: String) -> String {
    // Simulate async work
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    format!("Hello, {}! You've been greeted asynchronously!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_router::init(
            tauri_plugin_router::Router::new()
                .command("greet", greet) // Sync handler
                .command("async_greet", async_greet), // Async handler
        ))
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
