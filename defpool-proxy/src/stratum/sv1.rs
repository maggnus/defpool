use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stratum V1 message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sv1Message {
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Vec<Value>>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// Stratum V1 methods
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Sv1Method {
    Login,
    GetJob,
    Submit,
    KeepAlive,
    Unknown(String),
}

#[allow(dead_code)]
impl Sv1Method {
    pub fn from_str(s: &str) -> Self {
        match s {
            "login" => Self::Login,
            "getjob" => Self::GetJob,
            "submit" => Self::Submit,
            "keepalived" => Self::KeepAlive,
            _ => Self::Unknown(s.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Login => "login",
            Self::GetJob => "getjob",
            Self::Submit => "submit",
            Self::KeepAlive => "keepalived",
            Self::Unknown(s) => s,
        }
    }
}

impl Sv1Message {
    /// Parse from JSON line
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON line
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Get method if this is a request
    pub fn get_method(&self) -> Option<Sv1Method> {
        self.method.as_ref().map(|m| Sv1Method::from_str(m))
    }

    /// Check if this is a response
    pub fn is_response(&self) -> bool {
        self.result.is_some() || self.error.is_some()
    }

    /// Create a login request
    pub fn login(id: u64, wallet: &str, worker: &str) -> Self {
        Self {
            id: Some(Value::Number(id.into())),
            method: Some("login".to_string()),
            params: Some(vec![
                Value::String(format!("{}:{}", wallet, worker)),
            ]),
            result: None,
            error: None,
        }
    }

    /// Create a submit request
    pub fn submit(id: u64, job_id: &str, nonce: &str, result: &str) -> Self {
        Self {
            id: Some(Value::Number(id.into())),
            method: Some("submit".to_string()),
            params: Some(vec![
                Value::String(job_id.to_string()),
                Value::String(nonce.to_string()),
                Value::String(result.to_string()),
            ]),
            result: None,
            error: None,
        }
    }

    /// Create a job notification
    pub fn job(job_id: &str, blob: &str, target: &str, height: u64) -> Self {
        Self {
            id: None,
            method: Some("job".to_string()),
            params: Some(vec![
                Value::String(job_id.to_string()),
                Value::String(blob.to_string()),
                Value::String(target.to_string()),
                Value::Number(height.into()),
            ]),
            result: None,
            error: None,
        }
    }

    /// Create a success response
    pub fn ok_response(id: Value, result: Value) -> Self {
        Self {
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    #[allow(dead_code)]
    pub fn error_response(id: Value, code: i32, message: &str) -> Self {
        Self {
            id: Some(id),
            method: None,
            params: None,
            result: None,
            error: Some(serde_json::json!({
                "code": code,
                "message": message
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_login() {
        let json = r#"{"id":1,"method":"login","params":["wallet:worker"]}"#;
        let msg = Sv1Message::from_json(json).unwrap();
        assert_eq!(msg.get_method(), Some(Sv1Method::Login));
    }

    #[test]
    fn test_serialize_job() {
        let msg = Sv1Message::job("job1", "blob", "target", 100);
        let json = msg.to_json().unwrap();
        assert!(json.contains("job"));
    }
}
