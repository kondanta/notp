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
use util::read_from_stdin_securely;
// No more mod.rs thingy. Yay!
use operations::{
    add::add,
    delete::delete,
    get::get,
    list::list,
    Request,
};
use store::DataStore;

#[cfg(feature = "kv-store")]
use store::kv_store::SecretStore;

fn main() {
    let opt = Opt::get_cli_args();
    // --stdin || --key
    let key = if opt.stdin {
        Some(read_from_stdin_securely().unwrap_or_else(|_| "".to_owned()))
    } else {
        opt.key
    }
    .unwrap_or_else(|| "".to_owned());

    // Global store variable
    #[cfg(feature = "kv-store")]
    let store: SecretStore =
        DataStore::new().expect("Cannot create datastore object");

    // Parse command line args
    if let Some(name) = opt.get {
        let mc = new_magic_crypt!(&key, 256);
        let req = Request::new(Some(&name), store, Some(mc));
        let _ = get(req, opt.quiet);
    } else if opt.list {
        let req = Request::new(None, store, None);
        let _ = list(req);
    } else if let Some(name) = opt.add {
        let mc = new_magic_crypt!(&key, 256);
        let req = Request::new(Some(&name), store, Some(mc));
        let _ = add(req);
    } else if let Some(name) = opt.delete {
        let req = Request::new(Some(&name), store, None);
        let _ = delete(req);
    }
}
