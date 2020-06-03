use super::Request;
use crate::error::NotpResult;
use crate::store::DataStore;

pub(crate) fn delete<T: DataStore>(request: Request<'_, T>) -> NotpResult<()> {
    let store = request.store;
    let name = request.key.unwrap_or_default();

    match store.delete(String::from(name)) {
        Ok(_) => {
            println!("{} Deleted!", name);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
