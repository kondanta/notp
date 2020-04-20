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
    secrets: Vec<Vec<String>>,
}

impl<'a> Crypt<'a> {
    pub(crate) fn new() -> Self {
        Self {
            key:     "secretkey",
            secrets: Vec::new(),
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

            let mut test = Vec::new();
            test.push("fA2Sk4la9fHJZ9uvldR/5A==");
            test.push("QbalQ1uW8hNVEuRcFrhSDg==");
            test.push("KlqOy9dqAfNcieXSJeH5Gw==");

            self.secrets = s.lines()
                .map(|s| s.trim().split(',').map(String::from).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            println!("{:#?}", self.secrets);

            let eminem = self.secrets[0].iter().map(|data| {
                let mut mc: MagicCrypt = new_magic_crypt!(self.key, 256);
                mc.decrypt_base64_to_string(data.trim()).unwrap()
            }).collect::<Vec<_>>();

            println!("{:#?}", eminem);
            return Ok(());
        }
        Ok(())
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
