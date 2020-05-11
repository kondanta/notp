use crate::store::SecretStore;
use crate::util::read_from_stdin;
use magic_crypt::MagicCrypt;
use std::io::{
    stdout,
    Error,
    Write,
};

/// Adds new secret into data source
///
/// This function takes an identifier which I call `name` and AES `key`.
/// Then asks for the 2FA secret using stdin.
pub(crate) fn add(
    name: &str,
    key: &str,
) -> Result<(), Error> {
    let store = SecretStore::new().expect("Cannot get the store instance!");
    print!("Please enter the secret: ");

    // we need to flush stdout in order to fetch correct input from stdin.
    stdout().flush()?;

    let b64 = encrypt_value(key);

    match store.insert(String::from(name), b64) {
        Ok(_) => Ok(()),
        _ => panic!("Cannot insert secret!"),
    }
}

fn encrypt_value(enc_key: &str) -> String {
    let mut mc: MagicCrypt = new_magic_crypt!(enc_key, 256);
    let mut value = read_from_stdin().unwrap_or("".to_string());
    value.pop();
    mc.encrypt_str_to_base64(value)
}
