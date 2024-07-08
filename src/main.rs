use reqwest::Error;

mod http_client;
mod fake_credential_spammer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // for i in 0..10 {
    //     println!("mail: {}  password: {}", fake_credential_spammer::random_mail(), fake_credential_spammer::random_password());
    // }

    Ok(())
}