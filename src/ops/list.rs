use crate::error::NotpResult;
use crate::store::SecretStore;

/// Lists all existing identifiers.
///
/// It will only print the existing identifiers. Such as
/// ```
/// 1. Google
/// 2. Slack
/// 3. Jira
/// ```
pub(crate) fn list() -> NotpResult<()> {
    let store = SecretStore::new().expect("Cannot get the store instance!");

    store.list_secrets()
}
