use super::Request;
use crate::error::NotpResult;
use crate::store::DataStore;
use crate::util::read_from_stdin;
use magic_crypt::{
    MagicCrypt256,
    MagicCryptTrait,
};
use std::io::{
    stdout,
    Write,
};

/// Adds new secret into data source
///
/// This function takes an identifier which I call `name` and AES `key`.
/// Then asks for the 2FA secret using stdin.
pub(crate) fn add<T: DataStore>(request: Request<'_, T>) -> NotpResult<()> {
    let store: T = request.store;
    let mc = request
        .encryption_key
        .expect("Cannot get the encryption key!");
    let name = request.key.unwrap_or_default();

    print!("Please enter the secret: ");
    // we need to flush stdout in order to fetch correct input from stdin.
    stdout().flush()?;

    let b64 = encrypt_value(mc);

    match store.insert(String::from(name), b64) {
        Ok(_) => Ok(()),
        _ => panic!("Cannot insert secret!"),
    }
}

fn encrypt_value(mc: MagicCrypt256) -> String {
    let mut value = read_from_stdin().unwrap_or_else(|_| "".to_string());
    value.pop();
    mc.encrypt_str_to_base64(value)
}
