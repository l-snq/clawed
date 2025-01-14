// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fantoccini::{ClientBuilder, Locator};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn ping(something: &str) -> String {
    format!("piiiiiiiiiiiiiiing, {}", something)
}

#[tauri::command]
fn real() -> String {
    "hello".to_string()
}

#[tokio::main]
async fn scrape() -> Result<(), fantoccini::error::CmdError> {
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to initiate connection to web driver");
    
    client.goto("https://github.com/l-snq/").await?;
    let url = client.current_url().await?;

    assert_eq!(url.as_ref(), "https://github.com/l-snq");

    //https://github.com/l-snq?tab=repositories
    client.find(Locator::LinkText("Repositories")).await?;

    client.close().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
