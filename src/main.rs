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
use std::io::prelude::*;
use std::io::Error;

use crate::otp::TOTP;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
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

    let auth = TOTP::new(token);

    let code = auth.generate(
        30,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );

    print!("{}", SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64 / 30e9 as u64);

    //print!("{}", code);

    // create_file();
}
