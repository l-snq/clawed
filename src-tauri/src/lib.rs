// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fantoccini::{ClientBuilder, wd::Capabilities, Locator};
use serde_json::json;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn scrape() -> Result<(), fantoccini::error::CmdError> {
    // we are going to use capabilities to specify the browser to be headless.
    let mut caps = Capabilities::new();
    caps.insert("moz:firefoxOptions".to_string(), json!({
        "args": ["-headless"]
    }));
    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://127.0.0.1:4444")
        .await
        .expect("failed to initiate connection to web driver");
    
    client.goto("https://github.com/l-snq/").await?;
    let url = client.current_url().await?;

    assert_eq!(url.as_ref(), "https://github.com/l-snq");

    //https://github.com/l-snq?tab=repositories
    let repository = client.find(Locator::LinkText("Repositories")).await?;

    client.close().await
}

#[tauri::command]
fn real() {
    scrape().expect("wasn't able to scrape.");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![real])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
