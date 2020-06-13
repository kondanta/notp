use super::Request;
use crate::error::NotpResult;
use crate::store::DataStore;

pub(crate) fn delete<T: DataStore>(request: Request<'_, T>) -> NotpResult<()> {
    let store = request.store;
    let name = request.key.unwrap_or_default();

    store.delete(String::from(name))
}
