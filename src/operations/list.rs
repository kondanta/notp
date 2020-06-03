use super::Request;
use crate::error::NotpResult;
use crate::store::DataStore;

/// Lists all existing identifiers.
///
/// It will only print the existing identifiers. Such as
/// ```
/// 1. Google
/// 2. Slack
/// 3. Jira
/// ```
pub(crate) fn list<T: DataStore>(request: Request<'_, T>) -> NotpResult<()> {
    request.store.list()
}
