use dotenv::dotenv;
use fernet::DecryptionError;
use fernet::Fernet;
use std::env;

fn fernet() -> Fernet {
    dotenv().ok();
    fernet::Fernet::new(&env::var("FERNET_SECRET_KEY").unwrap()).unwrap()
}

pub fn encrypt(secret_phrase: &String) -> String {
    fernet().encrypt(secret_phrase.as_bytes())
}
pub fn decrypt(token: &String) -> Result<Vec<u8>, DecryptionError> {
    fernet().decrypt(token)
}
