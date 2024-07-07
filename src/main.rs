use http_client::get_request;
use reqwest::Error;

mod http_client;
mod fake_credential_spammer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    for i in 0..50 {
        println!("{}", fake_credential_spammer::random_mail());
    }

    Ok(())
}

