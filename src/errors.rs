use std::error::Error;

pub type BunnyStorageError = Box<dyn Error + Send + Sync + 'static>;