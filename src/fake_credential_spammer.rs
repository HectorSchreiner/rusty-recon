use crate::http_client;
use faker_rand::en_us::{internet::Email, names::FirstName};
use rand::Rng;
use passwords::PasswordGenerator;

pub fn send_post_request() {
    
}

pub fn random_mail() -> String {
    let mut rng = rand::thread_rng();

    let address = vec!["@yahoo.com", "@gmail.com", "@hotmail.com", "@gmail.com", "@gmail.com"];
    let name = rand::random::<Email>().to_string().chars().take_while(|&ch| ch != '@').collect::<String>();
    let address = address[rng.gen_range(0..address.len())];

    name + address
}

pub fn random_password() -> String {
    let mut rng = rand::thread_rng();

    let pass_generator = PasswordGenerator {
       length: rng.gen_range(1..5),
       numbers: true,
       lowercase_letters: true,
       uppercase_letters: false,
       symbols: false,
       spaces: false,
       exclude_similar_characters: false,
       strict: false,
    };
    let name = rand::random::<FirstName>().to_string();

    name + &pass_generator.generate_one().unwrap()
}
