use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

/// Minimal HTTP proxy for the frontend.
///
/// Why: WebView fetch can fail in some packaged Windows environments.
/// We do the networking in Rust (reqwest) and return the response.
#[tauri::command]
pub async fn http_request(
    method: String,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();

    let m = method
        .parse::<reqwest::Method>()
        .map_err(|e| format!("invalid method {method}: {e}"))?;

    let mut req = client.request(m, &url);

    if let Some(h) = headers {
        for (k, v) in h {
            // Skip invalid header names/values rather than hard-failing.
            if let (Ok(name), Ok(value)) = (
                reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                reqwest::header::HeaderValue::from_str(&v),
            ) {
                req = req.header(name, value);
            }
        }
    }

    if let Some(b) = body {
        req = req.body(b);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("network error calling {url}: {e}"))?;

    let status = resp.status().as_u16();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let body = resp
        .text()
        .await
        .map_err(|e| format!("failed reading response body: {e}"))?;

    Ok(HttpResponse {
        status,
        content_type,
        body,
    })
}

