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
use super::cli_args::OperationTypes;
use super::store::DataStore as DataStoreTrait;
use magic_crypt::MagicCrypt256;

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
pub(crate) struct Dispatcher<Store: DataStoreTrait> {
    store:          Store,
    mc:             MagicCrypt256,
    is_quiet:       bool,
    operation_type: OperationTypes,
}

impl<Store: DataStoreTrait> Dispatcher<Store> {
    /// Construtcs a new `Dispatcher` with given arguments.
    pub(crate) fn new(
        store: Store,
        mc: MagicCrypt256,
        operation_type: OperationTypes,
        is_quiet: bool,
    ) -> Self {
        Self {
            store,
            mc,
            operation_type,
            is_quiet,
        }
    }

    /// Performs a function call with using OperationType information that
    /// stored in `Dispatcher`.
    pub(crate) fn dispatcher(self) {
        match self.operation_type {
            OperationTypes::List => {
                list(Request::new(None, self.store, None)).ok();
            }
            OperationTypes::Delete(ref s) => {
                delete(Request::new(
                    Some(s.to_owned().as_ref()),
                    self.store,
                    None,
                ))
                .ok();
            }
            OperationTypes::Add(ref s) => {
                add(Request::new(
                    Some(s.to_owned().as_ref()),
                    self.store,
                    Some(self.mc),
                ))
                .ok();
            }
            OperationTypes::Get(ref s) => {
                get(
                    Request::new(
                        Some(s.to_owned().as_ref()),
                        self.store,
                        Some(self.mc),
                    ),
                    self.is_quiet,
                )
                .ok();
            }
            _ => println!("Will be implemented"),
        };
    }
}
