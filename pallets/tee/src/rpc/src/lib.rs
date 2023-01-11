pub use pallet_tee_runtime_api::TeeApi as TeeRuntimeApi;
use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{AccountId32, generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use tee_primitives::EnclaveRpc;

#[rpc(client, server)]
pub trait TeeApi<BlockHash> {
    #[method(name = "enclave_count")]
    fn enclave_count(&self, at: Option<BlockHash>) -> RpcResult<u32>;
    #[method(name = "enclave_select")]
    fn enclave_select(&self, at: Option<BlockHash>,need_count:u64) -> RpcResult<Vec<EnclaveRpc<AccountId32,Vec<u8>,Vec<u8>>>>;
}

/// A struct that implements the `TeeApi`.
pub struct TeePallet<C, Block> {
    // If you have more generics, no need to TeePallet<C, M, N, P, ...>
    // just use a tuple like TeePallet<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TeePallet<C, Block> {
    /// Create new `TeePallet` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}

impl<C, Block> TeeApiServer<<Block as BlockT>::Hash> for TeePallet<C, Block>
    where
        Block: BlockT,
        C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
        C::Api: TeeRuntimeApi<Block>,
{
    fn enclave_count(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));

        api.enclave_count(&at).map_err(runtime_error_into_rpc_err)
    }

    fn enclave_select(&self, at: Option<<Block as BlockT>::Hash>,need_count:u64) -> RpcResult<Vec<EnclaveRpc<AccountId32,Vec<u8>,Vec<u8>>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));

        api.enclave_select(&at,need_count).map_err(runtime_error_into_rpc_err)
    }
}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    CallError::Custom(ErrorObject::owned(
        RUNTIME_ERROR,
        "Runtime error",
        Some(format!("{:?}", err)),
    ))
        .into()
}