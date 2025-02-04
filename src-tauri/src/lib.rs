// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fantoccini::{wd::Capabilities, ClientBuilder, Locator};
use rand::{distr::Alphanumeric, Rng};

fn user_agent_gen() -> String {
    let mut s: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        println!("{}", s);
        s.insert_str(0, "--user-agent=");
        s
}

#[derive(Debug)]
struct AllElements {
    text: Vec<String>,
    link: Vec<String>,
    image: Vec<u8>,
}

#[tokio::main]
async fn scrape(state: &mut AllElements) -> Result<&mut AllElements, fantoccini::error::CmdError> {
    // refactor this to specifically scrape specific things
    // make 
    // scrapeImages()
    // scrapeLinks()
    // scrapeTexts()
    // and then pass in clientbuilder into each of those functions, run them in here.

    let mut caps = Capabilities::new();
    let chrome_opts = serde_json::json!({
        "args": [
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage",
            user_agent_gen(),
        ],
        "binary": "",
        "w3c": true
    });
    println!("chrome opts: {}", chrome_opts);

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


    let image = client.find(Locator::Css("img")).await?;
    let image_data = image.screenshot().await?;
    state.image = image_data;

    client.close().await;
    Ok(state)
}

fn processScrapeData() -> Vec<String> {
    //this will take the scraped data and convert it into an
    //array of strings, those are to be
    //rendered client side.
    // refactor this to specifically scrape specific things
    let mut elements = AllElements { text: vec![], link: vec![], image: vec![] };

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
    // refactor this to specifically scrape specific things
    processScrapeData()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scrapeDataCommand])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
