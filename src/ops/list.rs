use crate::file::Crypt;
use std::collections::HashMap;
use std::io::Error;

/// Lists all existing identifiers.
///
/// It will only print the existing identifiers. Such as
/// ```
/// 1. Google
/// 2. Slack
/// 3. Jira
/// ```
pub(crate) fn list(key: &str) -> Result<(), Error> {
    let mut c = Crypt::new(key);
    let _ = c.read()?;

    let mapped = mapify(&c.get_raw_data().to_vec(), "|");
    show(mapped);
    Ok(())
}

/// Takes vector and makes it HashMap.
fn mapify(
    data: &Vec<String>,
    pat: &str,
) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for element in data {
        let e: Vec<_> = element.split(&pat).collect();
        map.insert(String::from(e[0]), String::from(e[1]));
    }
    map
}

/// Prints the identifier data.
fn show(map: HashMap<String, String>) {
    let mut ctr = 1;
    for i in map {
        println!("{}. {}", ctr, i.0);
        ctr += 1;
    }
}
