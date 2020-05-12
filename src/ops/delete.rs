use crate::error::NotpResult;
use crate::store::SecretStore;

pub(crate) fn delete(name: &str) -> NotpResult<()> {
    let store = SecretStore::new().expect("Cannot get the store instance!");

    match store.delete(String::from(name)) {
        Ok(_) => {
            println!("{} Deleted!", name);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
