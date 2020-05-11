use crate::store::SecretStore;
use kv::Error;

/// Lists all existing identifiers.
///
/// It will only print the existing identifiers. Such as
/// ```
/// 1. Google
/// 2. Slack
/// 3. Jira
/// ```
pub(crate) fn list() -> Result<(), Error> {
    let store = SecretStore::new().expect("Cannot get the store instance!");

    store.list_secrets()
}
