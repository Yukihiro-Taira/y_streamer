/// Fire-and-forget: POST error to /api/errors/client. Never blocks the caller.
#[cfg(target_arch = "wasm32")]
pub fn report_client_error(bug_type: impl Into<String>, message: impl Into<String>, trace_id: Option<String>) {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::spawn_local;

    let bug_type = bug_type.into();
    let message = message.into();

    spawn_local(async move {
        let body = serde_json::json!({
            "bug_type": bug_type,
            "message": message,
            "trace_id": trace_id,
        });
        let body_str = match serde_json::to_string(&body) {
            Ok(s) => s,
            Err(_) => return,
        };

        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(&body_str));

        let Ok(req) = web_sys::Request::new_with_str_and_init("/api/errors/client", &opts) else {
            return;
        };
        let _ = req.headers().set("Content-Type", "application/json");

        if let Some(win) = web_sys::window() {
            let _ = wasm_bindgen_futures::JsFuture::from(win.fetch_with_request(&req)).await;
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn report_client_error(
    _bug_type: impl Into<String>,
    _message: impl Into<String>,
    _trace_id: Option<String>,
) {
}
