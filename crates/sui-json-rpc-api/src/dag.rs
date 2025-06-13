use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use sui_json_rpc_types::SuiDagBlock;
use sui_open_rpc_macros::open_rpc;

/// Read API for inspecting consensus DAG blocks.
///
/// The API is under the `suix` namespace and returns blocks across all
/// validators for the most recent rounds.
#[open_rpc(namespace = "suix", tag = "Consensus DAG API")]
#[rpc(server, client, namespace = "suix")]
pub trait DagReadApi {
    /// Return DAG blocks for recent rounds across all validators.
    ///
    /// * `num_rounds` - Number of rounds to fetch, defaults to 5 if not supplied.
    #[method(name = "getLatestDagBlocks")]
    async fn get_latest_dag_blocks(&self, num_rounds: Option<u64>) -> RpcResult<Vec<SuiDagBlock>>;
}
