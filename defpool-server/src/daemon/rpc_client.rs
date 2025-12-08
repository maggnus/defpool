use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, warn};

/// RPC client for communicating with coin daemons
pub struct DaemonRpcClient {
    client: reqwest::Client,
    rpc_url: String,
    rpc_user: Option<String>,
    rpc_password: Option<String>,
}

#[derive(Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<Value>,
}

#[derive(Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl DaemonRpcClient {
    pub fn new(rpc_url: String, rpc_user: Option<String>, rpc_password: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url,
            rpc_user,
            rpc_password,
        }
    }

    /// Make a JSON-RPC call to the daemon
    async fn call<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "defpool".to_string(),
            method: method.to_string(),
            params,
        };

        debug!("RPC call: {} to {}", method, self.rpc_url);

        let mut req = self.client.post(&self.rpc_url).json(&request);

        // Add basic auth if credentials provided
        if let (Some(user), Some(pass)) = (&self.rpc_user, &self.rpc_password) {
            req = req.basic_auth(user, Some(pass));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            warn!("RPC returned status: {}", response.status());
            anyhow::bail!("RPC error: {}", response.status());
        }

        let rpc_response: JsonRpcResponse<T> = response.json().await?;

        if let Some(error) = rpc_response.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        rpc_response
            .result
            .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))
    }

    /// Get block template for mining
    pub async fn get_block_template(&self, wallet_address: &str) -> Result<Value> {
        let params = vec![serde_json::json!({
            "wallet_address": wallet_address,
            "reserve_size": 8
        })];

        self.call("getblocktemplate", params).await
    }

    /// Submit a mined block
    pub async fn submit_block(&self, block_blob: &str) -> Result<String> {
        let params = vec![serde_json::json!(block_blob)];
        self.call("submitblock", params).await
    }

    /// Get blockchain info
    pub async fn get_info(&self) -> Result<Value> {
        self.call("getinfo", vec![]).await
    }

    /// Get network difficulty
    pub async fn get_difficulty(&self) -> Result<f64> {
        let info: Value = self.get_info().await?;
        
        // Try different field names for different coins
        if let Some(difficulty) = info.get("difficulty").and_then(|v| v.as_f64()) {
            return Ok(difficulty);
        }
        
        if let Some(difficulty) = info.get("diff").and_then(|v| v.as_f64()) {
            return Ok(difficulty);
        }

        anyhow::bail!("Could not extract difficulty from daemon response")
    }

    /// Get block count
    pub async fn get_block_count(&self) -> Result<u64> {
        let info: Value = self.get_info().await?;
        
        info.get("height")
            .or_else(|| info.get("blocks"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Could not extract block count from daemon response"))
    }

    /// Validate address
    pub async fn validate_address(&self, address: &str) -> Result<bool> {
        let params = vec![serde_json::json!(address)];
        let result: Value = self.call("validateaddress", params).await?;
        
        Ok(result
            .get("isvalid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running daemon
    async fn test_daemon_connection() {
        let client = DaemonRpcClient::new(
            "http://localhost:18081/json_rpc".to_string(),
            None,
            None,
        );

        let info = client.get_info().await;
        assert!(info.is_ok());
    }
}
