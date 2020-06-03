pub(crate) mod add;
pub(crate) mod delete;
pub(crate) mod get;
pub(crate) mod list;
use magic_crypt::MagicCrypt256;

pub(crate) struct Request<'a, DataStore> {
    key:            Option<&'a str>,
    store:          DataStore,
    encryption_key: Option<MagicCrypt256>,
}

impl<'a, DataStore> Request<'a, DataStore> {
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
