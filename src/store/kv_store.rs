use super::DataStore;
/// Keeps secrets in a file based KV storage.
use crate::error::{
    NotpError,
    NotpResult,
};
use crate::util::{
    create_folder,
    does_path_exists,
    get_folder_path,
};
use kv::{
    Bucket,
    Config,
    Store,
};

//#[derive(Clone)]
pub(crate) struct SecretStore {
    bucket: Bucket<'static, String, String>,
}

impl SecretStore {
    /// Creates the folder with given path.
    pub(self) fn init(path: &str) -> NotpResult<()> {
        create_folder(&path)
    }
}

impl DataStore for SecretStore {
    /// Creates new SecretStore instance.
    ///
    /// # Example
    /// ```rust
    /// use store::SecretStore;
    /// let store = SecretStore::new()?;
    /// ```
    fn new(
        mut path: Option<&str>,
        mut store_name: Option<&str>,
    ) -> NotpResult<Self> {
        let actual_path = path.get_or_insert("notp");
        if does_path_exists(actual_path) {
            SecretStore::init(actual_path)?;
        }
        let store = get_folder_path(actual_path)
            .map(|p| Store::new(Config::new(p)))
            .ok_or_else(|| {
                NotpError::Generic("Cannot create KV instance!".to_string())
            })?;
        let bucket = store?.bucket::<String, String>(Some(
            store_name.get_or_insert("store"),
        ))?;

        Ok(Self { bucket })
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
