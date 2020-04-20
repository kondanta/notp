use crate::util::get_folder_path;

use magic_crypt::MagicCrypt;

use std::fs::OpenOptions;
use std::io;
use std::io::{
    BufReader,
    Error,
    Read,
    Write,
};

pub(crate) struct Crypt<'a> {
    key:     &'a str,
    secrets: Vec<String>,
    raw_data: Vec<String>,
}

impl<'a> Crypt<'a> {
    pub(crate) fn new() -> Self {
        Self {
            key:     "secretkey",
            secrets: Vec::new(),
            raw_data: Vec::new(),
        }
    }

    pub(crate) fn write_into_file(&self) -> Result<(), Error> {
        // Key must be 16 chars
        let mut mc: MagicCrypt = new_magic_crypt!(self.key, 256);
        let mut line = read_secret()?;
        line.pop();

        if let Some(totp_file) = get_totp_file_path() {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(totp_file)?;
            let b64 = mc.encrypt_str_to_base64(line);
            file.write_all(b64.as_ref())
                .expect("Cannot write into file");
            file.write_all(",".as_ref())
                .expect("Cannot write into file");
            return Ok(());
        }
        Ok(())
    }

    pub(crate) fn read(&mut self) -> Result<(), Error> {
        if let Some(totp_file) = get_totp_file_path() {
            let file = OpenOptions::new().read(true).open(totp_file)?;
            let mut s: String = String::new();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut s)?;

            self.secrets = s.trim().split(",").map(|s| s.to_string()).collect();
            // remove last empty field right after the last element.
            self.secrets.pop();

            self.raw_data = self
                .secrets
                .iter()
                .map(|data| {
                    let mut mc: MagicCrypt = new_magic_crypt!(self.key, 256);
                    mc.decrypt_base64_to_string(data)
                        .unwrap_or(String::from(""))
                })
                .collect::<Vec<_>>();
            return Ok(());
        }
        Ok(())
    }

    pub(crate) fn get_raw_data(&self) -> &Vec<String> {
        &self.raw_data
    }

    pub(crate) fn get_encrypted_data(&self) -> &Vec<String> {
        &self.secrets
    }
}

pub(crate) fn read_secret() -> Result<String, Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn get_totp_file_path() -> Option<String> {
    if let Some(totp_folder_path) = get_folder_path("totp") {
        let totp_file = format!("{}/{}", totp_folder_path, "totp");
        return Some(totp_file);
    }
    None
}
