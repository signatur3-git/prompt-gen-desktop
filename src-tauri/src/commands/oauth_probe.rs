use reqwest;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthProbeResult {
    pub ok: bool,
    pub token_endpoint: String,
    pub error: Option<String>,
}

/// Probe the token endpoint from the Rust backend.
/// This helps diagnose TLS/proxy/network issues in environments where the WebView can't fetch
/// and DevTools are unavailable.
#[tauri::command]
pub async fn oauth_probe(token_endpoint: String) -> Result<OAuthProbeResult, String> {
    let client = reqwest::Client::new();

    // We intentionally do a simple GET. The endpoint may return 405/404/etc, but we mainly
    // care whether we can establish a TLS connection and get any HTTP response.
    match client.get(&token_endpoint).send().await {
        Ok(resp) => Ok(OAuthProbeResult {
            ok: true,
            token_endpoint,
            error: Some(format!(
                "probe_status={} content_type={}",
                resp.status().as_u16(),
                resp.headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
            )),
        }),
        Err(e) => Ok(OAuthProbeResult {
            ok: false,
            token_endpoint,
            error: Some(format!("{e}")),
        }),
    }
}
