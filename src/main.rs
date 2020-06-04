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

use cli_args::Opt;
use operations::Dispatcher;
use store::DataStore;
use util::read_from_stdin_securely;

#[cfg(feature = "kv-store")]
use store::kv_store::SecretStore;

fn main() {
    let opt = Opt::get_cli_args();
    let map = opt.export_functions();
    // Global store variable
    #[cfg(feature = "kv-store")]
    let store: SecretStore =
        DataStore::new().expect("Cannot create datastore object");

    // --stdin || --key
    let key = if opt.stdin {
        Some(read_from_stdin_securely().unwrap_or_else(|_| "".to_owned()))
    } else {
        opt.key.clone()
    }
    .unwrap_or_else(|| "".to_owned());

    let mc = new_magic_crypt!(&key, 256);
    // Dispatch correct function
    for (operation_type, is_invoked) in map {
        if is_invoked {
            let d = Dispatcher::new(
                store.clone(),
                mc.clone(),
                operation_type,
                opt.quiet,
            );
            d.dispatcher();
        }
    }
}
