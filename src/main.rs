#![deny(
    clippy::unwrap_used,
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

//! This tool simply generates One Time Password using the given, then saved,
//! 2FA secret code.
//!
//! It uses AES encryption to hide 2FA secret codes. Each request requires aes
//! key(--key option) So make sure you don't lose your key since this tool does
//! not save the aes key itself.

#[macro_use]
extern crate magic_crypt;

mod cli_args;
mod error;
mod operations;
mod otp;
mod store;
mod util;

use operations::Dispatcher;
use version_check::is_min_version;

fn main() {
    match is_min_version("1.40.0").unwrap_or(false) {
        true => Dispatcher::dispatch(),
        false => eprintln!("Please update Rust to 1.40.0 or above"),
    };
}
