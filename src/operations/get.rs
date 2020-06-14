use super::Request;
use crate::error::{
    NotpError,
    NotpResult,
};

use crate::otp::{
    OtpGenerator,
    OTP,
};
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
    let token = store.get(String::from(name))?;

    if token.is_empty() {
        eprintln!("Cannot find the token, exiting...!");
        std::process::exit(1);
    }

    let token = request
        .encryption_key
        .ok_or_else(|| {
            NotpError::Generic("Could not get encryption key.".to_string())
        })?
        .decrypt_base64_to_string(token)?;

    print_otp(&token, name, quiet)
}

// utility function for printing the OTP code.
fn print_otp(
    token: &str,
    name: &str,
    quiet: bool,
) -> NotpResult<()> {
    // BEWARE! Token contains /r on Windows. Haven't tried it on other platforms
    // but it will probably contain some EOL kind of delimeter.
    let otp = OtpGenerator::new(token.trim()).generate(0, 30);

    match otp {
        Ok(code) => {
            if quiet {
                print!("{}", code);
                Ok(())
            } else {
                println!("OTP code for the {}: {}", name, code);
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        get,
        Request,
    };
    use crate::operations::{
        add::add,
        delete::delete,
    };
    use crate::store::{
        kv_store::SecretStore,
        DataStore as DataStoreTrait,
    };
    use crate::util::{
        create_folder,
        remove_folder,
    };

    fn kv_init<'a>(
        key: Option<&'a str>,
        enc: &str,
        path: Option<&'a str>,
        store: Option<&'a str>,
    ) -> Request<'a, SecretStore> {
        #[cfg(feature = "kv-store")]
        let store: SecretStore = DataStoreTrait::new(path, store)
            .expect("Could not create DataStore");
        let mc = new_magic_crypt!(enc, 256);
        Request::new(key, store, Some(mc))
    }

    #[test]
    fn should_get_data() {
        let path = "TestGetData";
        let _ = create_folder(path);
        add(kv_init(
            Some("TestGetData"),
            "TestKey",
            Some(path),
            Some("Test"),
        ))
        .ok();
        let r = get(
            kv_init(Some("TestGetData"), "TestKey", Some(path), Some("Test")),
            false,
        );
        assert!(r.is_ok());
        delete(kv_init(
            Some("TestGetData"),
            "TestKey",
            Some(path),
            Some("Test"),
        ))
        .ok();
        remove_folder(path).ok();
    }
}
