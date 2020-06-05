use super::Request;
use crate::error::{
    NotpError,
    NotpResult,
};
use crate::otp::OTP;
use crate::store::DataStore;
use magic_crypt::MagicCryptTrait;

/// Generates OTP code with the demanded identifier.
///
/// # Example
/// ```rust
/// use crate::ops::get::get;
/// get("GoogleAccount-1", "superSecretAesKey");
/// // If GoogleAccount-1 exists, it will print something like (GoogleAccount-1, 123456)
/// ```
pub(crate) fn get<T: DataStore>(
    request: Request<'_, T>,
    quiet: bool,
) -> NotpResult<()> {
    let store = request.store;
    let name = request.key.unwrap_or_default();

    let token = match store.get(String::from(name)) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };

    if token.is_empty() {
        eprintln!("Cannot find the token, exiting...!");
        std::process::exit(1);
    }

    let token = request
        .encryption_key
        .ok_or(NotpError::Generic(
            "Could not get encryption key.".to_string(),
        ))?
        .decrypt_base64_to_string(token)?;

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
