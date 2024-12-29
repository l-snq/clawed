use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    client.goto("https://www.redflagdeals.com/in/edmonton/flyers/categories/groceries/").await?;
    let c_url = client.current_url().await?;
    assert_eq!(c_url.as_ref(), "https://www.redflagdeals.com/in/edmonton/flyers/categories/groceries/");

    client.find(Locator::LinkText("Hot Deals")).await?.click().await?;
    

    Ok(())
}
