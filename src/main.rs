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

use crate::file::{
    read_secret,
    Crypt,
};
use crate::otp::OTP;
#[macro_use]
extern crate magic_crypt;

mod cli_args;
mod file;
mod otp;
mod util;

fn main() {
    let token =
        "NRNM7KGFTR6SUMPBAEMBETM2WGKVUWHH6Y4VEGNPZON3GMVXBHFJV4PJZRFBUXWD";

    let otp = OTP::new(token);
    println!("{}", otp.generate_otp());

    let mut c = Crypt::new();

    // c.write_into_file();
    match c.read() {
        Ok(_) => println!("OK"),
        Err(e) => println!("Err: {}", e.to_string()),
    }

    // read();
}
