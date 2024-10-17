use std::path::Path;

// region:      --- MODULES
pub mod Disk;
// endregion:   --- MODULES

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StorageType{
    Disk,
    Memory
}

#[derive(Debug, PartialEq, Clone)]
pub struct Meta{
    storage: StorageType,
    destination: Option<String>
}

async fn storage(meta: Meta){
if meta.storage == StorageType::Disk{
        Disk::store(meta.destination);
    }
}