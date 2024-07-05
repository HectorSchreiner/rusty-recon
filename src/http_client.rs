use reqwest::{Error, IntoUrl, Url};

pub async fn get_request(url: impl IntoUrl) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    println!("Status: {}", response.status());
    
    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}