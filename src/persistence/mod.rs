mod file;

pub use file::FilePersistence;

use std::collections::HashMap;
use crate::core::error::Result;
use crate::core::store::StoreData;

pub trait Persistence: Send + Sync {
    fn save(&self, data: &HashMap<String, StoreData>) -> Result<()>;
    fn load(&self) -> Result<HashMap<String, StoreData>>;
}