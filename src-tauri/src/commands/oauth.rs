use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, EventTarget, State};
use tiny_http::{Header, HeaderField, Response, Server};

#[derive(Default)]
pub struct OAuthCallbackState {
    pub inner: Arc<Mutex<OAuthCallbackInner>>,
}

#[derive(Default)]
pub struct OAuthCallbackInner {
    pub active: bool,
    pub port: Option<u16>,
    pub last_callback_url: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthStartResponse {
    pub redirect_uri: String,
}

/// Start a loopback callback server on 127.0.0.1 with an OS-assigned port.
///
/// The server will accept the first request to /oauth/callback and:
/// - store the full callback URL
/// - emit an `oauth-callback` event to the frontend
/// - respond with a simple HTML page
///
/// This is primarily meant for DEV because Windows deep links can spawn a second instance.
#[tauri::command]
pub fn oauth_start_loopback(
    app: AppHandle,
    state: State<OAuthCallbackState>,
) -> Result<OAuthStartResponse, String> {
    let mut inner = state
        .inner
        .lock()
        .map_err(|_| "oauth state poisoned".to_string())?;
    if inner.active {
        // already active, just return existing redirect
        if let Some(port) = inner.port {
            return Ok(OAuthStartResponse {
                redirect_uri: format!("http://localhost:{}/oauth/callback", port),
            });
        }
        return Err("oauth loopback server already active".to_string());
    }

    // Bind on fixed loopback port (must match allowlisted redirect_uri in the marketplace client).
    let port: u16 = 51234;
    let server = Server::http(format!("localhost:{port}")).map_err(|e| {
        format!("failed to bind loopback server on localhost:{port} (is it already in use?): {e}")
    })?;

    inner.active = true;
    inner.port = Some(port);

    let state_arc = state.inner.clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        // server is blocking; handle one callback then exit
        let start = Instant::now();
        let timeout = Duration::from_secs(60 * 5);

        let req_opt = server.incoming_requests().next();
        if start.elapsed() <= timeout {
            if let Some(request) = req_opt {
                let url = format!("http://localhost:{}{}", port, request.url());

                if let Ok(mut inner) = state_arc.lock() {
                    inner.last_callback_url = Some(url.clone());
                    inner.active = false;
                    inner.port = None;
                }

                let _ = app_handle.emit_to(EventTarget::any(), "oauth-callback", url.clone());

                let html = r#"<!doctype html>
<html><head><meta charset=\"utf-8\"><title>RPG Desktop</title></head>
<body style=\"font-family: sans-serif;\">
<h2>Authentication complete</h2>
<p>You can close this tab and return to the RPG Desktop app.</p>
</body></html>"#;

                let content_type = Header {
                    field: HeaderField::from_bytes("Content-Type").unwrap(),
                    value: "text/html; charset=utf-8".parse().unwrap(),
                };

                let response = Response::from_string(html).with_header(content_type);
                let _ = request.respond(response);
                return;
            }
        }

        // timeout / exit cleanup
        if let Ok(mut inner) = state_arc.lock() {
            inner.active = false;
            inner.port = None;
        }
    });

    Ok(OAuthStartResponse {
        redirect_uri: format!("http://localhost:{}/oauth/callback", port),
    })
}

/// Optional cleanup if the user cancels the flow.
#[tauri::command]
pub fn oauth_cancel_loopback(state: State<OAuthCallbackState>) -> Result<(), String> {
    let mut inner = state
        .inner
        .lock()
        .map_err(|_| "oauth state poisoned".to_string())?;
    inner.active = false;
    inner.port = None;
    inner.last_callback_url = None;
    Ok(())
}
