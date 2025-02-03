// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fantoccini::{wd::Capabilities, ClientBuilder, Locator};

#[derive(Debug)]
struct AllElements {
    text: Vec<String>,
    link: Vec<String>,
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
            "--disable-dev-shm-usage",
            "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
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
    //https://www.flyers-on-line.com/current-weekly-flyers
    client.goto("https://www.flyers-on-line.com/current-weekly-flyers").await?;
    let url = client.current_url().await?;

    assert_eq!(url.as_ref(), "https://www.flyers-on-line.com/current-weekly-flyers");
    let elements = client.find_all(Locator::Css("*")).await?;

    for element in elements {
        if let Ok(value) = element.text().await {
            state.text.push(value);
        }
    }

    let links = client.find_all(Locator::Css("a")).await?;
    for link in links {
        if let Ok(value) = link.text().await {
            state.link.push(value);
        }
    }

    client.close().await;
    Ok(state)
}

fn processScrapeData() -> Vec<String> {
    //this will take the scraped data and convert it into an
    //array of strings, those are to be
    //rendered client side.
    let mut elements = AllElements { text: vec![], link: vec![] };

    scrape(&mut elements).expect("can't scrape");

    let mut string: Vec<String> = Vec::new();

    for i in elements.text {
        string.push(i);
    }

    string.retain(|s| !s.trim().is_empty()); // sanitize vector

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
