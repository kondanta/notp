use super::Request;
use crate::error::{
    NotpError,
    NotpResult,
};
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
    let name = request.key.unwrap_or_default();
    let mc = request.encryption_key.ok_or_else(|| {
        NotpError::Generic("Could not get encryption key.".to_string())
    })?;

    print!("Please enter the secret: ");
    // we need to flush stdout in order to fetch correct input from stdin.
    stdout().flush()?;
    let b64 = encrypt_value(mc);
    store.insert(String::from(name), b64)
}

fn encrypt_value(mc: MagicCrypt256) -> String {
    let mut value = read_from_stdin().unwrap_or_else(|_| "".to_string());
    value.pop();
    mc.encrypt_str_to_base64(value)
}
