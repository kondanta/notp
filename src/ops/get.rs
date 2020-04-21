use crate::file::Crypt;
use crate::otp::OTP;
use std::collections::HashMap;
use std::io::Error;

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
) -> Result<(), Error> {
    let mut c = Crypt::new(key);
    c.read()?;

    let map = mapify(&c.get_raw_data().to_vec(), "|", name);
    show(map);
    Ok(())
}

/// Prints the otp code
fn show(map: HashMap<String, String>) {
    if map.is_empty() {
        println!(
            "Could not find the service you want. Consider using --list to \
             check whether it exists or not"
        );
        return;
    }
    for e in map {
        let otp = OTP::new(&e.1);
        println!("OTP code for the {}: {}", e.0, otp.generate_otp(6, 0, 30))
    }
}

/// Takes vector and makes it HashMap.
fn mapify(
    data: &[String],
    pat: &str,
    target: &str,
) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for element in data {
        let e: Vec<_> = element.split(&pat).collect();
        if is_searched(e[0], target) {
            map.insert(String::from(e[0]), String::from(e[1]));
        }
    }
    map
}

/// Logic for searching the identifier.
///
/// This might be looking meaningless but there would be a chance for me to
/// change the searching algorithm, so having separate function for that is ok.
fn is_searched(
    value: &str,
    target: &str,
) -> bool {
    value == target
}

#[cfg(test)]
mod tests {
    use crate::ops::get::{
        is_searched,
        mapify,
    };

    #[test]
    fn is_searched_should_return_true() {
        let val = "some string";
        let target = "some string";

        assert!(is_searched(val, target))
    }

    #[test]
    fn vector_to_map() {
        let v = vec![String::from("Key2=Value"), String::from("Key=Value")];
        let map = mapify(&v, "=", "Key");
        for e in map {
            assert_eq!("Key", e.0);
            assert_eq!("Value", e.1);
        }
    }
}
