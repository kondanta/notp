use crate::util::get_folder_path;

use kv::{
    Bucket,
    Config,
    Error,
    Store,
};

/// Keeps secrets in a file based KV storage.
pub(crate) struct SecretStore {
    bucket: Bucket<'static, String, String>,
}

impl SecretStore {
    /// Creates new SecretStore instance.
    pub(crate) fn new() -> Result<Self, Error> {
        let path = match get_folder_path("notp") {
            Some(p) => p,
            None => {
                eprintln!("Cannot find the notp path!");
                std::process::exit(2);
            }
        };
        // TODO: Move these in `init` function.
        let cfg = Config::new(path);
        let store = Store::new(cfg)?;
        let bucket = store.bucket::<String, String>(Some("store"))?;
        Ok(Self { bucket })
    }

    /// Inserts new secret into storage
    pub(crate) fn insert(
        &self,
        secret_name: String,
        secret_value: String,
    ) -> Result<(), Error> {
        self.bucket.set(secret_name, secret_value)
    }

    /// Finds searched secret using its name.
    pub(crate) fn get(
        &self,
        secret_name: String,
    ) -> Result<String, Error> {
        match self.bucket.get(secret_name) {
            Ok(result) => {
                if let Some(r) = result {
                    return Ok(r);
                }
                Err(Error::InvalidConfiguration)
            }
            Err(e) => Err(e),
        }
    }

    /// Lists available secrets.
    pub(crate) fn list_secrets(&self) -> Result<(), Error> {
        if self.bucket.is_empty() {
            println!("You don't have any secrets.");
            return Ok(());
        }

        let mut ctr = 1;
        for item in self.bucket.iter() {
            let item = item?;
            let key: String = item.key()?;
            println!("{}. {}", ctr, key);
            ctr += 1;
        }
        Ok(())
    }

    /// Deletes selected secret from storage.
    pub(crate) fn delete(
        &self,
        secret_name: String,
    ) -> Result<(), Error> {
        self.bucket.remove(secret_name)
    }
}
