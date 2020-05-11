use crate::store::SecretStore;
pub(crate) fn delete(name: &str) {
    let store = SecretStore::new().expect("Cannot get the store instance!");

    match store.delete(String::from(name)) {
        Ok(_) => println!("{} Deleted!", name),
        Err(_) => eprintln!("Could not delete {} !", name),
    };
}
