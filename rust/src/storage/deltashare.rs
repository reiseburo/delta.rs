//! The Delta Share backend encapsulates the functionality necessary for using a 
//! Delta Sharing server (https://delta.io/sharing/) API as the DeltaTable storage
//! backend

use futures::Stream;
use std::pin::Pin;

use super::{ObjectMeta, StorageBackend, StorageError};

/// An object to encapsulate the Delta Share's URL
#[derive(Debug, PartialEq)]
pub struct DeltaShareObject<'a> {
    /// The URL of the share
    pub url: &'a str,
}

impl<'a> std::fmt::Display for DeltaShareObject<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

/// The DeltaShareBackend implements read-only behaviors for the Delta Sharing API
#[derive(Debug)]
pub struct DeltaShareBackend {
    url: String,
}

impl DeltaShareBackend {
    /// Instantiate a new backend
    pub fn new(obj: DeltaShareObject<'_>) -> Self {
        Self { url: String::from(obj.url) }
    }
}

#[async_trait::async_trait]
impl StorageBackend for DeltaShareBackend {
    async fn head_obj(&self, _path: &str) -> Result<ObjectMeta, StorageError> {
        Err(StorageError::UnsupportedOperation("head_obj will not work against a read-only Delta Share".to_string()))
    }

    async fn get_obj(&self, _path: &str) -> Result<Vec<u8>, StorageError> {
        Err(StorageError::UnsupportedOperation("get_obj will not work against a read-only Delta Share".to_string()))
    }

    async fn list_objs<'a>(
        &'a self,
        _path: &'a str,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ObjectMeta, StorageError>> + Send + 'a>>,
        StorageError,
    > {
        Err(StorageError::UnsupportedOperation("list_obj will not work against a read-only Delta Share".to_string()))
    }

    async fn put_obj(&self, _path: &str, _obj_bytes: &[u8]) -> Result<(), StorageError> {
        Err(StorageError::UnsupportedOperation("put_obj will not work against a read-only Delta Share".to_string()))
    }

    async fn rename_obj(&self, _src: &str, _dst: &str) -> Result<(), StorageError> {
        Err(StorageError::UnsupportedOperation("rename_obj will not work against a read-only Delta Share".to_string()))
    }

    async fn delete_obj(&self, _path: &str) -> Result<(), StorageError> {
        Err(StorageError::UnsupportedOperation("delete_obj will not work against a read-only Delta Share".to_string()))
    }
}
