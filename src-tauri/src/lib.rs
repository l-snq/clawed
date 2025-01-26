// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fantoccini::{wd::Capabilities, ClientBuilder, Locator};

#[derive(Debug)]
struct AllElements {
    text: Vec<String>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn scrape(state: &mut AllElements) -> Result<&mut AllElements, fantoccini::error::CmdError> {
    let mut caps = Capabilities::new();
    let chrome_opts = serde_json::json!({
        "args": [
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage"
        ],
        "binary": "",
        "w3c": true
    });

    caps.insert("goog:chromeOptions".to_string(), chrome_opts);

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://127.0.0.1:4444")
        .await
        .expect("failed to initiate connection to web driver");

    client.goto("https://github.com/l-snq/").await?;
    let url = client.current_url().await?;

    assert_eq!(url.as_ref(), "https://github.com/l-snq/");

    // https://github.com/l-snq?
    // [data-tab-item='repositories']
    // "a[data-tab-item='repositories']"
    let repository = client
        .find(Locator::Css("a[data-tab-item='repositories']"))
        .await?;
    let elements = client.find_all(Locator::Css("a")).await?;

    for element in elements {
        if let Ok(value) = element.text().await {
            state.text.push(value);
            println!("{:?}", state.text);
        }
    }

    client.close().await;
    Ok(state)
}

fn processScrapeData() -> Vec<String> {
    //this will take the scraped data and convert it into an
    //array of strings, those are to be
    //rendered client side.
    let mut elements = AllElements { text: vec![] };

    scrape(&mut elements).expect("can't scrape");

    let mut string: Vec<String> = Vec::new();

    for i in elements.text {
        string.push(i);
    }

    string
}

// https://github.com/tauri-apps/tauri/discussions/3913 look at this!!
#[tauri::command]
fn scrapeDataCommand() -> Vec<String> {
    processScrapeData()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![scrapeDataCommand])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
