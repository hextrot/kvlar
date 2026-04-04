//! SHIELD cloud approval backend.
//!
//! When the proxy evaluates a policy and gets `RequireApproval`, this backend
//! creates an escalation in SHIELD and polls for the human decision.

use std::time::Duration;

use kvlar_core::{ApprovalRequest, ApprovalResponse};

use crate::approval::{ApprovalBackend, ApprovalError};

const POLL_INTERVAL: Duration = Duration::from_secs(5);

/// Approval backend that integrates with SHIELD's escalation center.
///
/// When called, it:
/// 1. POSTs to `{shield_url}/api/v1/escalations` to register the escalation
/// 2. Polls `GET {shield_url}/api/v1/escalations/{id}` every 5 seconds
/// 3. Returns the human decision (approved / denied) or times out
pub struct ShieldApprovalBackend {
    shield_url: String,
    api_key: String,
    client: reqwest::Client,
    /// How long to wait for a human decision before treating as deny.
    timeout: Duration,
}

impl ShieldApprovalBackend {
    /// Creates a new SHIELD approval backend.
    ///
    /// # Arguments
    /// * `shield_url` — Base URL of the SHIELD instance (e.g., `https://app.kvlar.io`)
    /// * `api_key` — API key for authentication
    /// * `timeout` — How long to poll before timing out (default: 300s)
    pub fn new(shield_url: impl Into<String>, api_key: impl Into<String>, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        Self {
            shield_url: shield_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            client,
            timeout,
        }
    }
}

impl ApprovalBackend for ShieldApprovalBackend {
    fn request_approval(
        &self,
        request: &ApprovalRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ApprovalResponse, ApprovalError>> + Send + '_>> {
        let shield_url = self.shield_url.clone();
        let api_key = self.api_key.clone();
        let client = self.client.clone();
        let timeout = self.timeout;
        let request = request.clone();

        Box::pin(async move {
            // Step 1: Create escalation in SHIELD
            let create_url = format!("{}/api/v1/escalations", shield_url);
            let body = serde_json::json!({
                "actionType": "tool_call",
                "resource": request.tool_name,
                "parameters": request.tool_arguments,
                "ruleMatched": request.rule_id,
                "reason": request.reason,
                "timeoutSeconds": timeout.as_secs(),
            });

            let create_resp = client
                .post(&create_url)
                .bearer_auth(&api_key)
                .json(&body)
                .send()
                .await
                .map_err(|e| ApprovalError::Backend(format!("failed to create escalation: {e}")))?;

            if !create_resp.status().is_success() {
                return Err(ApprovalError::Backend(format!(
                    "SHIELD returned {} when creating escalation",
                    create_resp.status()
                )));
            }

            let created: serde_json::Value = create_resp
                .json()
                .await
                .map_err(|e| ApprovalError::Backend(format!("failed to parse create response: {e}")))?;

            let escalation_id = created["id"]
                .as_str()
                .ok_or_else(|| ApprovalError::Backend("missing id in escalation response".to_string()))?
                .to_string();

            // Step 2: Poll for decision
            let poll_url = format!("{}/api/v1/escalations/{}", shield_url, escalation_id);
            let deadline = std::time::Instant::now() + timeout;

            loop {
                tokio::time::sleep(POLL_INTERVAL).await;

                if std::time::Instant::now() >= deadline {
                    return Err(ApprovalError::Timeout(timeout));
                }

                let poll_resp = client
                    .get(&poll_url)
                    .bearer_auth(&api_key)
                    .send()
                    .await
                    .map_err(|e| ApprovalError::Backend(format!("poll request failed: {e}")))?;

                if !poll_resp.status().is_success() {
                    // Transient error — keep polling
                    continue;
                }

                let status_body: serde_json::Value = poll_resp
                    .json()
                    .await
                    .map_err(|e| ApprovalError::Backend(format!("failed to parse poll response: {e}")))?;

                let status = status_body["status"].as_str().unwrap_or("pending");
                let decision_reason = status_body["decisionReason"].as_str().map(str::to_string);

                match status {
                    "approved" => {
                        return Ok(ApprovalResponse::Approved);
                    }
                    "denied" => {
                        return Ok(ApprovalResponse::Denied {
                            reason: decision_reason.or_else(|| Some("Denied by human reviewer".to_string())),
                        });
                    }
                    "expired" => {
                        return Err(ApprovalError::Timeout(timeout));
                    }
                    _ => {
                        // Still pending — keep polling
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_backend_creation() {
        let backend = ShieldApprovalBackend::new(
            "https://app.kvlar.io",
            "kvlar_sk_test",
            Duration::from_secs(300),
        );
        assert_eq!(backend.shield_url, "https://app.kvlar.io");
        assert_eq!(backend.timeout, Duration::from_secs(300));
    }

    #[test]
    fn test_url_trailing_slash_stripped() {
        let backend = ShieldApprovalBackend::new(
            "https://app.kvlar.io/",
            "key",
            Duration::from_secs(60),
        );
        assert_eq!(backend.shield_url, "https://app.kvlar.io");
    }
}
