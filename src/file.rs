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

/// Data model for the file operations.
///
/// Our main operations are writing into file and reading it from there.
/// While we doing this, we need a `key`, also we need to store the data that
/// read from the file.
pub(crate) struct Crypt<'a> {
    /// AES key that is used for encryption and decryption.
    key:      &'a str,
    /// Encrypted list of the secrets. Such as `AKSJFKxzkcjkasd==`
    secrets:  Vec<String>,
    /// Decrypted list of the secrets with their identifiers. Such as
    /// `Google|SomeSecretCode`
    raw_data: Vec<String>,
}

impl<'a> Crypt<'a> {
    /// Returns to a new Crypt struct with empty secrets.
    pub(crate) fn new(key: &'a str) -> Self {
        Self {
            key,
            secrets: Vec::new(),
            raw_data: Vec::new(),
        }
    }

    /// Writes data into config file.
    pub(crate) fn write_into_file(
        &self,
        name: String,
    ) -> Result<(), Error> {
        // Key must be 16 chars
        let mut mc: MagicCrypt = new_magic_crypt!(self.key, 256);
        let line = read_from_stdin()?;
        let mut line = name + "|" + &line;
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

    /// Reads the content of the config file.
    pub(crate) fn read(&mut self) -> Result<(), Error> {
        if let Some(totp_file) = get_totp_file_path() {
            let file = OpenOptions::new().read(true).open(totp_file)?;
            let mut s: String = String::new();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut s)?;

            self.secrets = s.trim().split(',').map(|s| s.to_string()).collect();
            // remove last empty field right after the last element.
            self.secrets.pop();

            self.raw_data = self
                .secrets
                .iter()
                .map(|data| {
                    let mut mc: MagicCrypt = new_magic_crypt!(self.key, 256);
                    mc.decrypt_base64_to_string(data)
                        .unwrap_or_else(|_| String::from(""))
                })
                .collect::<Vec<_>>();
            return Ok(());
        }
        Ok(())
    }

    /// Returns to decrypted list of the secrets.
    pub(crate) fn get_raw_data(&self) -> &Vec<String> {
        &self.raw_data
    }

    /// Returns to the encrypted list of the secrets.
    #[allow(unused)]
    pub(crate) fn get_encrypted_data(&self) -> &Vec<String> {
        &self.secrets
    }
}

/// Reads text from stdin
pub(crate) fn read_from_stdin() -> Result<String, Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

/// Returns to the Path string of the TOTP config file.
///
/// We need to find the config file in order to perform write and read
/// operations.
fn get_totp_file_path() -> Option<String> {
    if let Some(totp_folder_path) = get_folder_path("totp") {
        let totp_file = format!("{}/{}", totp_folder_path, "totp");
        return Some(totp_file);
    }
    None
}
