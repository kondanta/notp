use crate::file::Crypt;
use crate::util::mapify;
use std::collections::HashMap;
use std::io::Error;

pub(crate) fn list(key: String) -> Result<(), Error> {
    let mut c = Crypt::new(key);
    let _ = c.read()?;

    let mapped = mapify(c.get_raw_data().to_vec(), "|");
    show(mapped);
    Ok(())
}

fn show(map: HashMap<String, String>) {
    let mut ctr = 1;
    for i in map {
        println!("{}. {}", ctr, i.0);
        ctr += 1;
    }
}
