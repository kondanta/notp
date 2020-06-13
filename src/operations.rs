pub(self) mod add;
pub(self) mod delete;
pub(self) mod get;
pub(self) mod list;

use self::{
    add::add,
    delete::delete,
    get::get,
    list::list,
};
use super::cli_args::{
    OperationList,
    Opt,
};
use super::error::NotpResult;
use super::store::DataStore as DataStoreTrait;
use super::util::read_from_stdin_securely;
use magic_crypt::MagicCrypt256;

#[cfg(feature = "kv-store")]
use super::store::kv_store::SecretStore;

/// Contains required data for individual operations to handle database
/// operations.
///
/// # Detailed Explanation
/// While performing database operations, we have to know our datastore, key
/// which is going to mutated(inserted, deleted or fetched), and encryption key
/// to encrypt data while performing insertions. Also decrypting data while
/// performing `get` requests.
pub(crate) struct Request<'a, DataStore> {
    key:            Option<&'a str>,
    store:          DataStore,
    encryption_key: Option<MagicCrypt256>,
}

impl<'a, DataStore> Request<'a, DataStore> {
    /// Construct a new `Request` with given parameters.
    /// # Examples
    /// ```Rust
    /// use operations::Request;
    /// let req = Request::new(key, store, encryption_key);
    /// assert_eq(req.key, key);
    /// ```
    pub(crate) fn new(
        key: Option<&'a str>,
        store: DataStore,
        encryption_key: Option<MagicCrypt256>,
    ) -> Self {
        Self {
            key,
            store,
            encryption_key,
        }
    }
}

/// Function dispatcher. Responsibles from calling correct function with correct
/// parameters
pub(crate) struct Dispatcher {}

impl Dispatcher {
    /// Performs a function call with using OperationType information that
    /// stored in `Dispatcher`.
    pub(crate) fn dispatch() {
        // Global store variable
        #[cfg(feature = "kv-store")]
        let store: SecretStore =
            DataStoreTrait::new().expect("Could not create DataStore");
        let opt = Opt::get_cli_args();

        match &opt.operations {
            Some(OperationList::Add(param)) => {
                let encryption_key_candidate = match param.stdin {
                    true => read_from_stdin_securely()
                        .unwrap_or_else(|_| "".to_string()),
                    false => param.key.as_deref().unwrap_or("").to_string(),
                };
                let encryption_key =
                    new_magic_crypt!(&encryption_key_candidate, 256);
                let req = Request::new(
                    Some(&param.name),
                    store,
                    Some(encryption_key),
                );

                add(req).ok();
            }
            Some(OperationList::Get(param)) => {
                // Dispatcher::perform_op(param, |_| get, store.clone());
                let encryption_key_candidate = match param.stdin {
                    true => read_from_stdin_securely()
                        .unwrap_or_else(|_| "".to_string()),
                    false => param.key.as_deref().unwrap_or("").to_string(),
                };
                let encryption_key =
                    new_magic_crypt!(&encryption_key_candidate, 256);
                let req = Request::new(
                    Some(&param.name),
                    store,
                    Some(encryption_key),
                );

                get(req, param.quiet).ok();
            }
            Some(OperationList::Delete(param)) => {
                let req = Request::new(Some(&param.name), store, None);
                delete(req).ok();
            }
            _ => {
                let req = Request::new(None, store, None);
                list(req).ok();
            }
        };
    }
}
