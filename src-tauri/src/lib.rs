// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            println!("Setup function called!");
            
            // Debug: print all available windows
            let windows = app.webview_windows();
            println!("Available windows: {:?}", windows.keys().collect::<Vec<_>>());
            
            // Get the main window - try different possible labels
            let window = app.get_webview_window("main")
                .or_else(|| app.get_webview_window("default"))
                .or_else(|| app.get_webview_window(""))
                .or_else(|| {
                    // Try to get the first window if others fail
                    windows.into_iter().next().map(|(_, w)| w)
                })
                .expect("Failed to get main window");
            
            println!("Found window with label: {:?}", window.label());
            
            // Set transparent title bar on macOS
            #[cfg(target_os = "macos")]
            {
                use tauri::TitleBarStyle;
                println!("Setting transparent title bar on macOS...");
                match window.set_title_bar_style(TitleBarStyle::Transparent) {
                    Ok(_) => println!("Transparent title bar set successfully!"),
                    Err(e) => println!("Failed to set transparent title bar: {:?}", e),
                }
            }
            
            #[cfg(not(target_os = "macos"))]
            {
                println!("Not on macOS, skipping transparent title bar");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
