use crate::otp::OTP;
use crate::store::SecretStore;
use magic_crypt::MagicCrypt;
use std::io::Error;

// store.insert(&name).ok();
// store.get(&name);
// store.list_secrets().ok();

/// Generates OTP code with the demanded identifier.
///
/// # Example
/// ```rust
/// use crate::ops::get::get;
/// get("GoogleAccount-1", "superSecretAesKey");
/// // If GoogleAccount-1 exists, it will print something like (GoogleAccount-1, 123456)
/// ```
pub(crate) fn get(
    name: &str,
    key: &str,
    quiet: bool,
) -> Result<(), Error> {
    let mut mc: MagicCrypt = new_magic_crypt!(key, 256);

    let store = SecretStore::new().expect("Cannot get the store instance!");

    let token = match store.get(String::from(name)) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };

    if token.is_empty() {
        eprintln!("Cannot find the token, exiting...!");
        std::process::exit(1);
    }

    let token = mc
        .decrypt_base64_to_string(token)
        .expect("Cannot decrypt the token!");

    print_otp(&token, name, quiet);

    Ok(())
}

// utility function for printing the OTP code.
fn print_otp(
    token: &str,
    name: &str,
    quiet: bool,
) {
    let otp = OTP::new(&token);
    if quiet {
        print!("{}", otp.generate_otp(6, 0, 30))
    } else {
        println!("OTP code for the {}: {}", name, otp.generate_otp(6, 0, 30))
    }
}
