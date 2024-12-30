use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    client.goto("https://google.com").await?;

    let c_url = client.current_url().await?;
    assert_eq!(c_url.as_ref(), "https://google.com");

    client.find(Locator::LinkText("Francais")).await?.click().await?;

    Ok(())
}
