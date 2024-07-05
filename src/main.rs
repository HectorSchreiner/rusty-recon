use http_client::get_request;
use reqwest::Error;

mod http_client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.fruityvice.com/api/fruit/apple";
    get_request(url).await?;
    Ok(())
}

