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

#[macro_use]
extern crate magic_crypt;

mod cli_args;
mod file;
mod ops;
mod otp;
mod util;

use crate::cli_args::Opt;
use crate::ops::{
    add::add,
    get::get,
    list::list,
};

fn main() {
    let opt = Opt::get_cli_args();
    let key = &opt.key;

    if let Some(name) = opt.get {
        let _ = get(&name, key);
    } else if opt.list {
        let _ = list(key);
    } else if let Some(name) = opt.add {
        let _ = add(name, key);
    }
}
