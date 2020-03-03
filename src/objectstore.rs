use crate::{ObjectStore, Result};
use wapc_guest::host_call;
use wascc_codec::blobstore::Blob;
use wascc_codec::blobstore::Container;
use wascc_codec::blobstore::{BlobList, FileChunk, StreamRequest, Transfer};
use wascc_codec::blobstore::{
    OP_CREATE_CONTAINER, OP_GET_OBJECT_INFO, OP_LIST_OBJECTS, OP_REMOVE_CONTAINER,
    OP_REMOVE_OBJECT, OP_START_DOWNLOAD, OP_START_UPLOAD, OP_UPLOAD_CHUNK,
};
use wascc_codec::{deserialize, serialize};

/// The reserved capability ID for a key/value store. Used for call routing in the host runtime.
pub const CAPID_BLOBSTORE: &str = "wascc:blobstore";

/// An abstraction around a host runtime capability for a key-value store
#[derive(Default)]
pub struct DefaultObjectStore {}

impl DefaultObjectStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ObjectStore for DefaultObjectStore {
    fn create_container(&self, name: &str) -> Result<Container> {
        let cmd = Container {
            id: name.to_string(),
        };
        host_call(CAPID_BLOBSTORE, OP_CREATE_CONTAINER, &serialize(cmd)?)
            .map(|v| deserialize::<Container>(v.as_ref()).unwrap())
            .map_err(|e| e.into())
    }

    fn remove_container(&self, name: &str) -> Result<()> {
        let cmd = Container {
            id: name.to_string(),
        };
        host_call(CAPID_BLOBSTORE, OP_REMOVE_CONTAINER, &serialize(cmd)?)
            .map(|_v| ())
            .map_err(|e| e.into())
    }

    fn remove_object(&self, name: &str, container: &str) -> crate::Result<()> {
        let cmd = Blob {
            id: name.to_string(),
            container: container.to_string(),
            byte_size: 0,
        };
        host_call(CAPID_BLOBSTORE, OP_REMOVE_OBJECT, &serialize(cmd)?)
            .map(|_v| ())
            .map_err(|e| e.into())
    }

    fn list_objects(&self, container: &str) -> Result<BlobList> {
        let cmd = Container {
            id: container.to_string(),
        };
        host_call(CAPID_BLOBSTORE, OP_LIST_OBJECTS, &serialize(cmd)?)
            .map(|v| deserialize::<BlobList>(v.as_ref()).unwrap())
            .map_err(|e| e.into())
    }

    fn get_blob_info(&self, container: &str, id: &str) -> Result<Option<Blob>> {
        let cmd = Blob {
            id: id.to_string(),
            container: container.to_string(),
            byte_size: 0,
        };
        host_call(CAPID_BLOBSTORE, OP_GET_OBJECT_INFO, &serialize(cmd)?)
            .map(|v| {
                let b = deserialize::<Blob>(v.as_ref()).unwrap();
                if b.id.is_empty() {
                    None
                } else {
                    Some(b)
                }
            })
            .map_err(|e| e.into())
    }

    fn start_upload(&self, blob: &Blob, chunk_size: u64, total_bytes: u64) -> Result<Transfer> {
        let transfer = Transfer {
            blob_id: blob.id.to_string(),
            container: blob.container.to_string(),
            chunk_size,
            total_size: total_bytes,
            total_chunks: total_bytes / chunk_size,
        };
        let cmd = FileChunk {
            sequence_no: 0,
            container: blob.container.to_string(),
            id: blob.id.to_string(),
            chunk_size,
            total_bytes,
            chunk_bytes: vec![],
        };
        host_call(CAPID_BLOBSTORE, OP_START_UPLOAD, &serialize(cmd)?)
            .map(|_v| transfer)
            .map_err(|e| e.into())
    }

    fn upload_chunk(&self, transfer: &Transfer, offset: u64, bytes: &[u8]) -> crate::Result<()> {
        let cmd = FileChunk {
            id: transfer.blob_id.to_string(),
            container: transfer.container.to_string(),
            sequence_no: offset,
            chunk_size: transfer.chunk_size,
            total_bytes: transfer.total_size,
            chunk_bytes: bytes.to_vec(),
        };
        host_call(CAPID_BLOBSTORE, OP_UPLOAD_CHUNK, &serialize(cmd)?)
            .map(|_v| ())
            .map_err(|e| e.into())
    }

    fn start_download(&self, blob: &Blob, chunk_size: u64) -> crate::Result<Transfer> {
        let transfer = Transfer {
            blob_id: blob.id.to_string(),
            container: blob.container.to_string(),
            chunk_size,
            total_size: blob.byte_size,
            total_chunks: blob.byte_size / chunk_size,
        };
        let cmd = StreamRequest {
            container: blob.container.to_string(),
            id: blob.id.to_string(),
            chunk_size,
        };
        host_call(CAPID_BLOBSTORE, OP_START_DOWNLOAD, &serialize(cmd)?)
            .map(|_v| transfer)
            .map_err(|e| e.into())
    }
}
