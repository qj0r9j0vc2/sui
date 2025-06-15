// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;
use sui_json_rpc::SuiRpcModule;
use sui_json_rpc_api::{DagReadApiClient, DagReadApiServer};
use sui_json_rpc_types::SuiDagBlock;
use sui_open_rpc::Module;

use crate::errors::client_error_to_error_object;

pub(crate) struct DagReadApi {
    client: DagReadApiClient<HttpClient>,
}

impl DagReadApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            client: DagReadApiClient::new(fullnode_client),
        }
    }
}

#[async_trait]
impl DagReadApiServer for DagReadApi {
    async fn get_latest_dag_blocks(&self, num_rounds: Option<u64>) -> RpcResult<Vec<SuiDagBlock>> {
        self.client
            .get_latest_dag_blocks(num_rounds)
            .await
            .map_err(client_error_to_error_object)
    }
}

impl SuiRpcModule for DagReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        sui_json_rpc_api::DagReadApiOpenRpc::module_doc()
    }
}
