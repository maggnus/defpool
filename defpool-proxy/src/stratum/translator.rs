use super::sv1::{Sv1Message, Sv1Method};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, warn};

/// Translator between Stratum V1 and V2 protocols
pub struct StratumTranslator {
    /// Map SV1 job IDs to SV2 job IDs
    job_id_map: Arc<Mutex<HashMap<String, String>>>,
    /// Current job counter
    job_counter: Arc<Mutex<u64>>,
    /// Wallet address for this connection
    wallet_address: Option<String>,
    /// Worker name for this connection
    worker_name: Option<String>,
}

impl StratumTranslator {
    pub fn new() -> Self {
        Self {
            job_id_map: Arc::new(Mutex::new(HashMap::new())),
            job_counter: Arc::new(Mutex::new(0)),
            wallet_address: None,
            worker_name: None,
        }
    }

    /// Handle SV1 login message
    pub fn handle_sv1_login(&mut self, msg: &Sv1Message) -> Result<Sv1Message> {
        debug!("Handling SV1 login");

        // Extract wallet and worker from params
        if let Some(params) = &msg.params {
            if let Some(Value::String(login)) = params.first() {
                let parts: Vec<&str> = login.split(':').collect();
                self.wallet_address = Some(parts[0].to_string());
                self.worker_name = parts.get(1).map(|s| s.to_string());

                debug!(
                    "Login: wallet={}, worker={:?}",
                    self.wallet_address.as_ref().unwrap(),
                    self.worker_name
                );
            }
        }

        // Create success response
        let response = Sv1Message::ok_response(
            msg.id.clone().unwrap_or(serde_json::Value::Null),
            serde_json::json!({
                "id": "proxy_connection",
                "job": {
                    "job_id": "initial",
                    "blob": "",
                    "target": "00000000"
                },
                "status": "OK"
            }),
        );

        Ok(response)
    }

    /// Handle SV1 submit message
    pub fn handle_sv1_submit(&self, msg: &Sv1Message) -> Result<Sv1Message> {
        debug!("Handling SV1 submit");

        // Extract submit parameters
        if let Some(params) = &msg.params {
            if params.len() >= 3 {
                let job_id = params[0].as_str().unwrap_or("");
                let nonce = params[1].as_str().unwrap_or("");
                let result = params[2].as_str().unwrap_or("");

                debug!("Submit: job_id={}, nonce={}, result={}", job_id, nonce, result);

                // TODO: Translate to SV2 share submission
                // For now, accept the share
                let response = Sv1Message::ok_response(
                    msg.id.clone().unwrap_or(serde_json::Value::Null),
                    serde_json::json!({"status": "OK"}),
                );

                return Ok(response);
            }
        }

        // Invalid submit
        Ok(Sv1Message::error_response(
            msg.id.clone().unwrap_or(serde_json::Value::Null),
            -1,
            "Invalid submit parameters",
        ))
    }

    /// Generate a new SV1 job ID
    pub fn generate_job_id(&self) -> String {
        let mut counter = self.job_counter.lock().unwrap();
        *counter += 1;
        format!("job_{}", *counter)
    }

    /// Map SV2 job to SV1 job
    pub fn map_job(&self, sv2_job_id: String, sv1_job_id: String) {
        let mut map = self.job_id_map.lock().unwrap();
        map.insert(sv1_job_id, sv2_job_id);
    }

    /// Get SV2 job ID from SV1 job ID
    pub fn get_sv2_job_id(&self, sv1_job_id: &str) -> Option<String> {
        let map = self.job_id_map.lock().unwrap();
        map.get(sv1_job_id).cloned()
    }

    /// Get wallet address
    pub fn wallet_address(&self) -> Option<&str> {
        self.wallet_address.as_deref()
    }

    /// Get worker name
    pub fn worker_name(&self) -> Option<&str> {
        self.worker_name.as_deref()
    }

    /// Process SV1 message from miner
    pub fn process_sv1_message(&mut self, msg: &Sv1Message) -> Result<Option<Sv1Message>> {
        match msg.get_method() {
            Some(Sv1Method::Login) => Ok(Some(self.handle_sv1_login(msg)?)),
            Some(Sv1Method::Submit) => Ok(Some(self.handle_sv1_submit(msg)?)),
            Some(Sv1Method::KeepAlive) => {
                // Respond to keepalive
                Ok(Some(Sv1Message::ok_response(
                    msg.id.clone().unwrap_or(serde_json::Value::Null),
                    serde_json::json!({"status": "KEEPALIVED"}),
                )))
            }
            Some(Sv1Method::GetJob) => {
                // Miner requesting new job
                warn!("GetJob not implemented yet");
                Ok(None)
            }
            Some(Sv1Method::Unknown(method)) => {
                warn!("Unknown SV1 method: {}", method);
                Ok(Some(Sv1Message::error_response(
                    msg.id.clone().unwrap_or(serde_json::Value::Null),
                    -1,
                    &format!("Unknown method: {}", method),
                )))
            }
            None => {
                // Response message, ignore
                Ok(None)
            }
        }
    }

    /// Create SV1 job notification from mining data
    pub fn create_sv1_job(&self, blob: &str, target: &str, height: u64) -> Sv1Message {
        let job_id = self.generate_job_id();
        Sv1Message::job(&job_id, blob, target, height)
    }
}

impl Default for StratumTranslator {
    fn default() -> Self {
        Self::new()
    }
}

use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_handling() {
        let mut translator = StratumTranslator::new();
        let login_msg = Sv1Message::login(1, "wallet123", "worker1");
        
        let response = translator.handle_sv1_login(&login_msg).unwrap();
        assert!(response.is_response());
        assert_eq!(translator.wallet_address(), Some("wallet123"));
        assert_eq!(translator.worker_name(), Some("worker1"));
    }

    #[test]
    fn test_job_id_mapping() {
        let translator = StratumTranslator::new();
        translator.map_job("sv2_job_1".to_string(), "sv1_job_1".to_string());
        
        assert_eq!(
            translator.get_sv2_job_id("sv1_job_1"),
            Some("sv2_job_1".to_string())
        );
    }
}
