pub(crate) mod kv_store;
use crate::error::NotpResult;

pub(crate) trait DataStore: Sized {
    fn new() -> NotpResult<Self>;
    fn insert(
        &self,
        key: String,
        value: String,
    ) -> NotpResult<()>;
    fn get(
        &self,
        key: String,
    ) -> NotpResult<String>;
    fn list(&self) -> NotpResult<()>;
    fn delete(
        &self,
        key: String,
    ) -> NotpResult<()>;
}
