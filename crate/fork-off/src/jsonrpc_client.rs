use std::future::Future;

use async_channel::{bounded, Receiver, Sender};
use backoff::{future::retry, ExponentialBackoff};
use jsonrpc_core::Error;
use jsonrpc_core_client::{transports::ws, RpcError};
use jsonrpc_derive::rpc;

use crate::types::{BlockHash, ChildStorageMap, StorageKey, StorageValue};

#[rpc]
pub trait Rpc {
    #[rpc(name = "chain_getBlockHash")]
    fn get_block_hash(&self, block_number: Option<u32>) -> Result<BlockHash, Error>;

    #[rpc(name = "state_getStorage")]
    fn get_storage(
        &self,
        key: StorageKey,
        block_hash: Option<BlockHash>,
    ) -> Result<StorageValue, Error>;

    #[rpc(name = "state_getKeysPaged")]
    fn get_keys_paged(
        &self,
        prefix: StorageKey,
        count: usize,
        start_key: Option<StorageKey>,
        at: Option<BlockHash>,
    ) -> Result<Vec<StorageKey>, Error>;

    #[rpc(name = "childstate_getKeysPaged")]
    fn get_child_keys_paged(
        &self,
        child_storage: StorageKey,
        prefix: StorageKey,
        count: usize,
        start_key: Option<StorageKey>,
        at: Option<BlockHash>,
    ) -> Result<Vec<StorageKey>, Error>;

    #[rpc(name = "childstate_getStorageEntries")]
    fn get_child_storage_entries(
        &self,
        child_storage: StorageKey,
        keys: Vec<StorageKey>,
        at: Option<BlockHash>,
    ) -> Result<Vec<StorageValue>, Error>;
}

type RpcResult<T> = Result<T, RpcError>;

/// A JSONRPC aleph client that uses a websocket transport.
///
/// The WS transport makes it easy to cheaply multiplex requests within a single connection.
pub struct Client {
    /// This inner client type is generated by the `#[rpc]` macro.
    client: gen_client::Client,
}

const CHUNK_SIZE: usize = 1000usize;
const STORAGE_CAP: usize = 10 * CHUNK_SIZE;

impl Client {
    /// Connect to the given websocket endpoint (eg. `"wss://ws.test.azero.dev"`).
    pub async fn new(endpoint: &str) -> RpcResult<Client> {
        Ok(Client {
            client: ws::try_connect(endpoint)?.await?,
        })
    }

    /// Find the hash of the best known block.
    pub async fn best_block(&self) -> RpcResult<BlockHash> {
        self.client.get_block_hash(None).await
    }

    /// Fetchers all keys in the `at` block.
    ///
    /// Returns a `(Receiver<Storage>, fetcher)` pair. The `fetcher` must be `await`ed on to begin
    /// fetching. Then, `StorageKeys` can be taken out of the `Receiver`.
    pub fn stream_all_keys(
        &self,
        at: &BlockHash,
    ) -> (
        Receiver<StorageKey>,
        impl Future<Output = RpcResult<()>> + '_,
    ) {
        let (sender, receiver) = bounded(STORAGE_CAP);
        (receiver, self.do_stream_all_keys(sender, at.clone()))
    }

    /// Returns a map representing a single child trie
    pub async fn get_child_storage_for_key(
        &self,
        child_key: StorageKey,
        at: &BlockHash,
    ) -> RpcResult<Option<ChildStorageMap>> {
        let res = self
            .get_child_storage_for_key_inner(child_key, at)
            .await
            .map(Some);

        if let Err(RpcError::JsonRpcError(err)) = res {
            // Empty child storage is returned as error
            if err.message == "Client error: Invalid child storage key" {
                Ok(None)
            } else {
                Err(RpcError::JsonRpcError(err))
            }
        } else {
            res
        }
    }

    async fn get_child_storage_for_key_inner(
        &self,
        child_key: StorageKey,
        at: &BlockHash,
    ) -> RpcResult<ChildStorageMap> {
        let empty_prefix = StorageKey::new("0x");
        let mut child_storage_map = ChildStorageMap::new();
        let mut start_key = None;

        loop {
            let keys = self
                .client
                .get_child_keys_paged(
                    child_key.clone(),
                    empty_prefix.clone(),
                    CHUNK_SIZE,
                    start_key,
                    Some(at.clone()),
                )
                .await?;

            let values = self
                .client
                .get_child_storage_entries(child_key.clone(), keys.clone(), Some(at.clone()))
                .await?;

            child_storage_map.append(&mut keys.iter().cloned().zip(values).collect());

            let fetched = keys.len();
            start_key = keys.last().cloned();

            if fetched < CHUNK_SIZE {
                break;
            }
        }

        Ok(child_storage_map)
    }

    async fn do_stream_all_keys(&self, sender: Sender<StorageKey>, at: BlockHash) -> RpcResult<()> {
        let empty_prefix = StorageKey::new("0x");
        let mut start_key = None;

        loop {
            let keys = self
                .client
                .get_keys_paged(
                    empty_prefix.clone(),
                    CHUNK_SIZE,
                    start_key,
                    Some(at.clone()),
                )
                .await?;

            let fetched = keys.len();
            start_key = keys.last().cloned();

            for key in keys {
                sender.send(key).await.unwrap();
            }

            if fetched < CHUNK_SIZE {
                break;
            }
        }

        Ok(())
    }

    /// Fetch the value under `key` in the `at` block.
    pub async fn get_storage(&self, key: StorageKey, at: BlockHash) -> RpcResult<StorageValue> {
        retry(ExponentialBackoff::default(), || async {
            self.client
                .get_storage(key.clone(), Some(at.clone()))
                .await
                .map_err(backoff::Error::transient)
        })
        .await
    }
}