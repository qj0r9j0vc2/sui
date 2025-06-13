use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use parking_lot::RwLock;
use consensus_core::DagState;
use sui_open_rpc::Module;
use sui_json_rpc_api::{DagReadApiOpenRpc, DagReadApiServer, JsonRpcMetrics};
use sui_json_rpc_types::SuiDagBlock;

use crate::{with_tracing, SuiRpcModule, error::Error};

#[derive(Clone)]
pub struct DagReadApi {
    dag_state: Option<Arc<RwLock<DagState>>>,
    _metrics: Arc<JsonRpcMetrics>,
}

impl DagReadApi {
    pub fn new(dag_state: Option<Arc<RwLock<DagState>>>, metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { dag_state, _metrics: metrics }
    }
}

#[async_trait]
impl DagReadApiServer for DagReadApi {
    async fn get_latest_dag_blocks(&self, num_rounds: Option<u64>) -> RpcResult<Vec<SuiDagBlock>> {
        with_tracing!(async move {
            let Some(ds) = &self.dag_state else {
                return Err(Error::UnsupportedFeature("DAG state unavailable".to_string()).into());
            };
            let num_rounds = num_rounds.unwrap_or(5);
            let ds = ds.read();
            let highest = ds.highest_accepted_round();
            let start = highest.saturating_sub(num_rounds as u32);
            let mut blocks = Vec::new();
            for i in 0..ds.committee().size() {
                let author = ds.committee().to_authority_index(i).unwrap();
                let mut b = ds.get_cached_blocks_in_range(author, start, highest + 1, usize::MAX);
                blocks.append(&mut b);
            }
            Ok(blocks.into_iter().map(SuiDagBlock::from).collect())
        })
    }
}

impl SuiRpcModule for DagReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        DagReadApiOpenRpc::module_doc()
    }
}
