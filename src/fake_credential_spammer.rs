use crate::http_client;
use faker_rand::en_us::internet::Email;
use rand::Rng;

pub fn send_post_request() {
    
}

pub fn random_mail() -> String {
    let mut rng = rand::thread_rng();

    let address = vec!["@yahoo.com", "@gmail.com", "@hotmail.com", "@gmail.com", "@gmail.com"];
    let name = rand::random::<Email>().to_string().chars().take_while(|&ch| ch != '@').collect::<String>();
    let address = address[rng.gen_range(0..address.len())];

    name + address
}
