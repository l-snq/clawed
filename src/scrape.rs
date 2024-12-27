use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
pub async fn scraper() -> Result<(), fantoccini::error::CmdError> {
    let client = ClientBuilder::native()
        .connect("127.0.0.1:4444")
        .await
        .expect("failed to connect to the client");
    client.goto("https://www.redflagdeals.com/in/edmonton/flyers/categories/groceries/").await?;

    let url = client.current_url().await?;
    assert_eq!(url.as_ref(), "https://www.redflagdeals.com/in/edmonton/flyers/categories/groceries/");

    client.find(Locator::LinkText("Hot Deals")).await?.click().await?;

    client.close().await
}
