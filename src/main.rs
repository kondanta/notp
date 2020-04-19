#![deny(
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::get_last_with_len,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts
)]
#![warn(rust_2018_idioms)]

//! some doc

mod cli_args;
mod otp;
mod util;

use crate::util::get_folder_path;
use std::io::Error;

use oath::{
    totp_raw_now,
    HashType,
};

use std::{
    fs,
    io,
};

fn read_secret() -> Result<String, Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn create_file() -> Result<(), Error> {
    let line = read_secret()?;
    if let Some(totp_folder_path) = get_folder_path("totp") {
        let totp_file = format!("{}/{}", totp_folder_path, "totp");
        fs::write(totp_file, line).expect("Cannot write into file");
        return Ok(());
    }
    Ok(())
}

fn main() {
    let token =
        "NRNM7KGFTR6SUMPBAEMBETM2WGKVUWHH6Y4VEGNPZON3GMVXBHFJV4PJZRFBUXWD";

    let decode_token =
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, token)
            .unwrap();

    let auth = totp_raw_now(&decode_token, 6, 0, 30, &HashType::SHA1);

    // print!("{}", utc);

    print!("{}", auth);

    // create_file();
}
