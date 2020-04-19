use crate::util::get_folder_path;

use magic_crypt::MagicCrypt;

use std::fs::{
    File,
    OpenOptions,
};
use std::io::{
    BufRead,
    BufReader,
    Error,
    Read,
    Write,
};
use std::{
    fs,
    io,
};

pub(crate) struct Crypt {
    key: String,
}

impl Crypt {
    pub(crate) fn new() -> Self {
        Self { key: String::new() }
    }
}

pub(crate) fn read_secret() -> Result<String, Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

pub(crate) fn write_into_file() -> Result<(), Error> {
    // Key must be 16 chars
    let mut mc: MagicCrypt = new_magic_crypt!("magickey", 256);
    let line = read_secret()?;
    if let Some(totp_file) = get_totp_file_path() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(totp_file)?;
        let b64 = mc.encrypt_str_to_base64(line);
        file.write(b64.as_ref()).expect("Cannot write into file");
        return Ok(());
    }
    Ok(())
}

pub(crate) fn read() -> Result<(), Error> {
    let mut mc: MagicCrypt = new_magic_crypt!("magickey", 256);
    if let Some(totp_file) = get_totp_file_path() {
        let mut file =
            OpenOptions::new().read(true).to_owned().open(totp_file)?;

        let mut s = String::new();
        file.read_to_string(&mut s);
        let res: Vec<String> = s.split("=").map(|s| s.to_string()).collect();
        let mut v: Vec<String> = Vec::new();
        for mut data in res {
            data = data + "=";
            println!("{}", data);
            if data.len() > 3 {
                println!("Decrypted: {}", mc.decrypt_base64_to_string(data).unwrap());
                //v.push(mc.decrypt_base64_to_string(data.trim()))
            }
        }
        println!("{:#?}", v);
        return Ok(());
    }
    Ok(())
}

fn get_totp_file_path() -> Option<String> {
    if let Some(totp_folder_path) = get_folder_path("totp") {
        let totp_file = format!("{}/{}", totp_folder_path, "totp");
        return Some(totp_file);
    }
    None
}
