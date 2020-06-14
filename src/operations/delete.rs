use super::Request;
use crate::error::NotpResult;
use crate::store::DataStore;

pub(crate) fn delete<T: DataStore>(request: Request<'_, T>) -> NotpResult<()> {
    let store = request.store;
    let name = request.key.unwrap_or_default();

    store.delete(String::from(name))
}

#[cfg(test)]
mod tests {
    use super::{
        delete,
        Request,
    };
    use crate::operations::add::add;
    use crate::store::{
        kv_store::SecretStore,
        DataStore as DataStoreTrait,
    };
    use crate::util::{
        create_folder,
        remove_folder,
    };

    fn kv_init<'a>(
        key: Option<&'a str>,
        enc: &str,
        path: Option<&'a str>,
        store: Option<&'a str>,
    ) -> Request<'a, SecretStore> {
        #[cfg(feature = "kv-store")]
        let store: SecretStore = DataStoreTrait::new(path, store)
            .expect("Could not create DataStore");
        let mc = new_magic_crypt!(enc, 256);
        Request::new(key, store, Some(mc))
    }

    #[test]
    fn should_delete_data() {
        let path = "TestDeleteData";
        let _ = create_folder(path);
        add(kv_init(
            Some("TestDeleteData"),
            "TestKey",
            Some(path),
            Some("Test"),
        ))
        .ok();
        let r = delete(kv_init(
            Some("TestDeleteData"),
            "TestKey",
            Some(path),
            Some("Test"),
        ));
        assert!(r.is_ok());
        delete(kv_init(
            Some("TestGetData"),
            "TestKey",
            Some(path),
            Some("Test"),
        ))
        .ok();
        remove_folder(path).ok();
    }
}
