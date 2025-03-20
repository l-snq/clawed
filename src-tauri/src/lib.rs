use fantoccini::{error::CmdError, wd::Capabilities, Client, ClientBuilder, Locator};
use rand::{distr::Alphanumeric, Rng};
use serde::{ser::Serializer, Deserializer, Serialize};
use thiserror;
use anyhow;

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
    image: Vec<Vec<u8>>,
}

#[tokio::main]
async fn scrape_links(client: Client, state: &mut AllElements) -> Result<&mut AllElements, fantoccini::error::CmdError> {
    let elements = client.find_all(Locator::Css("a")).await?;

    for element in elements {
        if let Ok(value) = element.text().await {
            state.link.push(value);
        }
    }
    Ok(state)
}

#[tokio::main]
async fn scrape_image(client: Client, state: &mut AllElements) -> Result<&mut AllElements, fantoccini::error::CmdError> {
    let elements = client.find_all(Locator::Css("img")).await?;

    for element in elements {
        if let Ok(value) = element.screenshot().await {
            // this is expecting u8 instead of Vec<u8>??
            state.image.push(value);
        }
    }
    Ok(state)
}

#[tokio::main]
async fn scrape_text(client: Client, state: &mut AllElements) -> Result<&mut AllElements, fantoccini::error::CmdError> {
    let elements = client.find_all(Locator::Css("*")).await?;

    for element in elements {
        if let Ok(value) = element.text().await {
            state.text.push(value);
            state.text.retain(|s| !s.trim().is_empty());
        }
    }
    Ok(state)
}

#[tokio::main]
async fn scrape(state: &mut AllElements) -> CommandResult<&mut AllElements, fantoccini::error::CmdError> {
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
    scrape_text(client.clone(), state).expect("failed to scrape text");
    scrape_links(client.clone(), state).expect("failed to scrape links");
    scrape_image(client.clone(), state).expect("failed to scrape images");

    client.close().await.expect("something went wrong trying to close the client");
    Ok(state)
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    // make some error here
    Io(#[from] std::io::Error)
}

// have to manually implement serialize for our error.
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where 
            S: Serializer,
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;
// https://github.com/tauri-apps/tauri/discussions/3913 look at this!!
#[tauri::command]
fn scrape_data_command() -> CommandResult<AllElements> {
   let mut elements = AllElements { text: vec![], link: vec![], image: vec![] };
   scrape(&mut elements)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scrape_data_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
