use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;

const MAX_SESSION_LIFETIME_SECONDS: u64 = 3_600;

pub struct GatewaySessionStore {
    session: Mutex<Option<GatewaySession>>,
}

impl Default for GatewaySessionStore {
    fn default() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }
}

#[derive(Clone)]
struct GatewaySession {
    base_url: Url,
    access_token: String,
    subscription_id: String,
    expires_at: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindGatewaySessionInput {
    pub base_url: String,
    pub access_token: String,
    pub subscription_id: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewaySessionStatus {
    pub configured: bool,
    pub authenticated: bool,
    pub subscription_id: Option<String>,
    pub expires_at: Option<u64>,
    pub base_origin: Option<String>,
    pub credential_storage: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedCapabilityRequest {
    pub project_id: String,
    pub job_id: String,
    pub approval_fingerprint: String,
    pub idempotency_key: String,
    pub requested_width: u32,
    pub requested_height: u32,
    pub panorama_mode: String,
    pub output_count: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedCapabilityResponse {
    pub request_fingerprint: String,
    pub entitlement_status: String,
    pub capability_status: String,
    pub model_id: Option<String>,
    pub supported_width: Option<u32>,
    pub supported_height: Option<u32>,
    pub panorama_mode: Option<String>,
    pub price_status: String,
    pub price_amount_minor: Option<u64>,
    pub price_currency: Option<String>,
    pub credit_cost: Option<u64>,
    pub quota_status: String,
    pub quota_remaining: Option<u64>,
    pub checked_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedGenerationInput {
    pub role: String,
    pub checksum_sha256: String,
    pub data_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedGenerationRequest {
    pub project_id: String,
    pub job_id: String,
    pub approval_fingerprint: String,
    pub idempotency_key: String,
    pub requested_width: u32,
    pub requested_height: u32,
    pub panorama_mode: String,
    pub output_count: u8,
    pub inputs: Vec<ManagedGenerationInput>,
    pub prompt: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedGenerationOutput {
    pub b64_json: String,
    pub revised_prompt: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedGenerationResponse {
    pub request_fingerprint: String,
    pub approval_fingerprint: String,
    pub idempotency_key: String,
    pub provider_request_id: Option<String>,
    pub status: String,
    pub output_count: u8,
    pub outputs: Vec<ManagedGenerationOutput>,
    pub usage: Option<serde_json::Value>,
    pub completed_at: u64,
}

fn validate_base_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|_| "Managed gateway URL is invalid.".to_string())?;
    if url.scheme() != "https" || url.host_str().is_none() {
        return Err("Managed gateway must use an HTTPS origin.".into());
    }
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
    {
        return Err("Managed gateway URL must be an HTTPS origin without credentials, paths, query parameters, or fragments.".into());
    }
    Ok(url)
}

fn validate_session_input(
    input: BindGatewaySessionInput,
    now: u64,
) -> Result<GatewaySession, String> {
    let base_url = validate_base_url(&input.base_url)?;
    let token = input.access_token.trim();
    if token.len() < 24 || token.chars().any(char::is_whitespace) {
        return Err("Managed gateway access token is invalid.".into());
    }
    if input.subscription_id.trim().is_empty() || input.subscription_id.len() > 120 {
        return Err("Managed subscription identifier is invalid.".into());
    }
    if input.expires_at <= now
        || input.expires_at > now.saturating_add(MAX_SESSION_LIFETIME_SECONDS)
    {
        return Err("Managed gateway token must be short-lived and expire within one hour.".into());
    }
    Ok(GatewaySession {
        base_url,
        access_token: token.into(),
        subscription_id: input.subscription_id.trim().into(),
        expires_at: input.expires_at,
    })
}

impl GatewaySessionStore {
    pub fn bind(
        &self,
        input: BindGatewaySessionInput,
        now: u64,
    ) -> Result<GatewaySessionStatus, String> {
        let session = validate_session_input(input, now)?;
        let status = session_status(Some(&session), now);
        *self
            .session
            .lock()
            .map_err(|_| "Managed gateway session lock is unavailable.".to_string())? =
            Some(session);
        Ok(status)
    }

    pub fn clear(&self) -> Result<(), String> {
        *self
            .session
            .lock()
            .map_err(|_| "Managed gateway session lock is unavailable.".to_string())? = None;
        Ok(())
    }

    pub fn status(&self, now: u64) -> Result<GatewaySessionStatus, String> {
        let session = self
            .session
            .lock()
            .map_err(|_| "Managed gateway session lock is unavailable.".to_string())?;
        Ok(session_status(session.as_ref(), now))
    }

    fn active_session(&self, now: u64) -> Result<GatewaySession, String> {
        let session = self
            .session
            .lock()
            .map_err(|_| "Managed gateway session lock is unavailable.".to_string())?;
        let session = session
            .as_ref()
            .ok_or_else(|| "Managed subscription is not signed in.".to_string())?;
        if session.expires_at <= now {
            return Err(
                "Managed subscription session expired; sign in again before any provider request."
                    .into(),
            );
        }
        Ok(session.clone())
    }

    pub async fn capability(
        &self,
        request: &ManagedCapabilityRequest,
        now: u64,
    ) -> Result<ManagedCapabilityResponse, String> {
        let session = self.active_session(now)?;
        let endpoint = session
            .base_url
            .join("v1/desktop/capabilities")
            .map_err(|_| "Managed gateway capability endpoint is invalid.".to_string())?;
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .map_err(|error| format!("Unable to prepare managed gateway client: {error}"))?;
        let response = client
            .post(endpoint)
            .bearer_auth(&session.access_token)
            .header("X-Estate-Subscription", &session.subscription_id)
            .json(request)
            .send()
            .await
            .map_err(|error| format!("Managed gateway capability check failed: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "Managed gateway rejected capability check with HTTP {}.",
                response.status().as_u16()
            ));
        }
        response
            .json::<ManagedCapabilityResponse>()
            .await
            .map_err(|error| {
                format!("Managed gateway returned an invalid capability response: {error}")
            })
    }

    pub async fn submit_image(
        &self,
        request: &ManagedGenerationRequest,
        now: u64,
    ) -> Result<ManagedGenerationResponse, String> {
        let session = self.active_session(now)?;
        let endpoint = session
            .base_url
            .join("v1/desktop/images")
            .map_err(|_| "Managed gateway image endpoint is invalid.".to_string())?;
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|error| format!("Unable to prepare managed gateway client: {error}"))?;
        let response = client
            .post(endpoint)
            .bearer_auth(&session.access_token)
            .header("X-Estate-Subscription", &session.subscription_id)
            .json(request)
            .send()
            .await
            .map_err(|error| {
                format!("Managed gateway image request outcome is unknown: {error}")
            })?;
        if !response.status().is_success() {
            return Err(format!(
                "Managed gateway rejected image submission with HTTP {}.",
                response.status().as_u16()
            ));
        }
        response
            .json::<ManagedGenerationResponse>()
            .await
            .map_err(|error| format!("Managed gateway returned an invalid image response: {error}"))
    }
}

fn session_status(session: Option<&GatewaySession>, now: u64) -> GatewaySessionStatus {
    let Some(session) = session else {
        return GatewaySessionStatus {
            configured: false,
            authenticated: false,
            subscription_id: None,
            expires_at: None,
            base_origin: None,
            credential_storage: "memory_only".into(),
            detail: "Managed OpenAI subscription is not configured.".into(),
        };
    };
    let authenticated = session.expires_at > now;
    GatewaySessionStatus {
        configured: true,
        authenticated,
        subscription_id: Some(session.subscription_id.clone()),
        expires_at: Some(session.expires_at),
        base_origin: Some(session.base_url.origin().ascii_serialization()),
        credential_storage: "memory_only".into(),
        detail: if authenticated {
            "Short-lived managed subscription session is active in memory.".into()
        } else {
            "Managed subscription session expired; sign in again.".into()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_contract_requires_https_and_short_lived_token() {
        let valid = BindGatewaySessionInput {
            base_url: "https://gateway.example.test/".into(),
            access_token: "short-lived-session-token-value".into(),
            subscription_id: "subscription-test".into(),
            expires_at: 1_300,
        };
        let session = validate_session_input(valid, 1_000).unwrap();
        assert_eq!(
            session.base_url.origin().ascii_serialization(),
            "https://gateway.example.test"
        );
        assert!(validate_session_input(
            BindGatewaySessionInput {
                base_url: "http://gateway.example.test/".into(),
                access_token: "short-lived-session-token-value".into(),
                subscription_id: "subscription-test".into(),
                expires_at: 1_300,
            },
            1_000
        )
        .is_err());
        assert!(validate_session_input(
            BindGatewaySessionInput {
                base_url: "https://user:pass@gateway.example.test/".into(),
                access_token: "short-lived-session-token-value".into(),
                subscription_id: "subscription-test".into(),
                expires_at: 1_300,
            },
            1_000
        )
        .is_err());
        assert!(validate_session_input(
            BindGatewaySessionInput {
                base_url: "https://gateway.example.test/".into(),
                access_token: "short-lived-session-token-value".into(),
                subscription_id: "subscription-test".into(),
                expires_at: 5_000,
            },
            1_000
        )
        .is_err());
    }

    #[test]
    fn public_gateway_status_never_serializes_access_token() {
        let store = GatewaySessionStore::default();
        let status = store
            .bind(
                BindGatewaySessionInput {
                    base_url: "https://gateway.example.test/".into(),
                    access_token: "short-lived-session-token-value".into(),
                    subscription_id: "subscription-test".into(),
                    expires_at: 1_300,
                },
                1_000,
            )
            .unwrap();
        let encoded = serde_json::to_string(&status).unwrap();
        assert!(!encoded.contains("short-lived-session-token-value"));
        assert!(!encoded.contains("accessToken"));
        assert_eq!(status.credential_storage, "memory_only");
    }
}
