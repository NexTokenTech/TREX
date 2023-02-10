/*
	Copyright 2022 NexToken Tech LLC

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/
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
use tee_primitives::Enclave;

#[rpc(client, server)]
pub trait TeeApi<BlockHash> {
    #[method(name = "enclave_count")]
    fn enclave_count(&self, at: Option<BlockHash>) -> RpcResult<u32>;
    #[method(name = "enclave_select")]
    fn enclave_select(&self,need_count:u64) -> RpcResult<Vec<Enclave<AccountId32,Vec<u8>,Vec<u8>>>>;
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

    fn enclave_select(&self, need_count:u64) -> RpcResult<Vec<Enclave<AccountId32,Vec<u8>,Vec<u8>>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let enclave_count = api.enclave_count(&at).unwrap_or(0u32);
        let res = api.enclave_select(&at,need_count).unwrap_or(vec![]);
        return if res.len() == 0 {
            if enclave_count == 0 {
                Err(CallError::Custom(ErrorObject::owned(
                    RpcErrorType::NoEnclaveIsInRegistry as i32,
                    "No enclave is registered",
                    Some(""),
                )).into())
            }else{
                Err(CallError::Custom(ErrorObject::owned(
                    RpcErrorType::NotEnoughEnclaveError as i32,
                    "Not enough enclave accounts",
                    Some(""),
                )).into())
            }

        } else {
            Ok(res)
        };
    }
}

enum RpcErrorType {
    RuntimeError = 1,
    NotEnoughEnclaveError = 2,
    NoEnclaveIsInRegistry = 3
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    CallError::Custom(ErrorObject::owned(
        RpcErrorType::RuntimeError as i32,
        "Runtime error",
        Some(format!("{:?}", err)),
    ))
        .into()
}