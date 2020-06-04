use super::DataStore;
/// Keeps secrets in a file based KV storage.
use crate::error::{
    NotpError,
    NotpResult,
};
use crate::util::get_folder_path;
use kv::{
    Bucket,
    Config,
    Store,
};

#[derive(Clone)]
pub(crate) struct SecretStore {
    bucket: Bucket<'static, String, String>,
}

impl DataStore for SecretStore {
    /// Creates new SecretStore instance.
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// let store = SecretStore::new()?;
    /// ```
    fn new() -> NotpResult<Self> {
        if let Some(path) = get_folder_path("notp") {
            let cfg = Config::new(path);
            let store = Store::new(cfg)?;
            let bucket = store.bucket::<String, String>(Some("store"))?;
            return Ok(Self { bucket });
        };
        Err(NotpError::Generic(String::from(
            "Cannot find the notp folder!",
        )))
    }

    /// Inserts new secret into storage
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// // get store instance using new() function.
    /// store.insert(String::from("Key"), String::from("Value"))
    /// ```
    fn insert(
        &self,
        secret_name: String,
        secret_value: String,
    ) -> NotpResult<()> {
        self.bucket
            .set(secret_name, secret_value)
            .map_err(Into::into)
    }

    /// Finds searched secret using its name.
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// // get store instance using new() function.
    /// store.get(String::from("Key"))
    /// ```
    fn get(
        &self,
        secret_name: String,
    ) -> NotpResult<String> {
        match self.bucket.get(secret_name) {
            Ok(result) => {
                if let Some(r) = result {
                    return Ok(r);
                }
                Err(NotpError::Kv(kv::Error::InvalidConfiguration))
            }
            Err(e) => Err(NotpError::Kv(e)),
        }
    }

    /// Lists available secrets.
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// // get store instance using new() function.
    /// store.list_secrets()
    /// ```
    fn list(&self) -> NotpResult<()> {
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
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// // get store instance using new() function.
    /// store.delete(String::from("Key"))
    /// ```
    fn delete(
        &self,
        secret_name: String,
    ) -> NotpResult<()> {
        self.bucket.remove(secret_name).map_err(Into::into)
    }
}
