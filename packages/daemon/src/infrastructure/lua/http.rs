//! `waio.http` module — safe HTTP access for Lua widgets.
//!
//! Only exposed when the aura declares `permissions: [http]` in its `.wa` file.
//! Uses `ureq` (synchronous, no async runtime needed) with URL validation,
//! 10s timeout, and 10MB max response size.

use mlua::prelude::*;
use std::time::Duration;

use super::rate_limiter::RateLimiter;

const MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024; // 10 MB
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// HTTP access configuration for a widget.
#[derive(Clone)]
pub struct HttpAccess {
    /// Allowed hosts. If empty, all hosts are allowed (with scheme validation).
    allowed_hosts: Vec<String>,
    /// Maximum response size in bytes.
    max_response_size: usize,
    /// Request timeout.
    timeout: Duration,
}

impl HttpAccess {
    pub fn new(allowed_hosts: Vec<String>) -> Self {
        Self {
            allowed_hosts,
            max_response_size: MAX_RESPONSE_SIZE,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    fn is_url_allowed(&self, url: &str) -> bool {
        let parsed = match url::Url::parse(url) {
            Ok(u) => u,
            Err(_) => return false,
        };

        // Only http/https.
        if !["http", "https"].contains(&parsed.scheme()) {
            return false;
        }

        // Check host allowlist.
        let host = parsed.host_str().unwrap_or("").to_string();
        if self.allowed_hosts.is_empty() {
            true // All hosts allowed.
        } else {
            self.allowed_hosts.contains(&host)
        }
    }

    fn build_agent(&self) -> ureq::Agent {
        ureq::AgentBuilder::new().timeout(self.timeout).build()
    }
}

/// Create the `waio.http` Lua module with `get(url)` and `post(url, body, content_type?)`.
pub fn create_module(
    lua: &Lua,
    http_access: HttpAccess,
    rate_limiter: Option<RateLimiter>,
) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    // GET: waio.http.get(url) -> { status, body }
    let http_for_get = http_access.clone();
    let rl_for_get = rate_limiter.clone();
    m.set(
        "get",
        lua.create_function(move |lua, url: String| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_for_get {
                rl.check_and_record("http")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }

            if !http_for_get.is_url_allowed(&url) {
                return Err(mlua::Error::external(format!("URL not allowed: {}", url)));
            }

            let agent = http_for_get.build_agent();
            let resp = agent
                .get(&url)
                .call()
                .map_err(|e| mlua::Error::external(e.to_string()))?;

            let status: u16 = resp.status();
            let body = resp
                .into_string()
                .map_err(|e| mlua::Error::external(e.to_string()))?;

            if body.len() > http_for_get.max_response_size {
                return Err(mlua::Error::external(format!(
                    "Response too large: {} bytes (max {})",
                    body.len(),
                    http_for_get.max_response_size
                )));
            }

            let result = lua.create_table()?;
            result.set("status", status as i64)?;
            result.set("body", body)?;
            Ok(result)
        })?,
    )?;

    // POST: waio.http.post(url, body, content_type?) -> { status, body }
    let http_for_post = http_access;
    let rl_for_post = rate_limiter;
    m.set(
        "post",
        lua.create_function(
            move |lua,
                  (url, body, content_type): (String, String, Option<String>)|
                  -> LuaResult<LuaTable> {
                if let Some(ref rl) = rl_for_post {
                    rl.check_and_record("http")
                        .map_err(|e| mlua::Error::external(e.to_string()))?;
                }

                if !http_for_post.is_url_allowed(&url) {
                    return Err(mlua::Error::external(format!("URL not allowed: {}", url)));
                }

                let agent = http_for_post.build_agent();
                let mut req = agent.post(&url);
                if let Some(ct) = content_type {
                    req = req.set("Content-Type", &ct);
                }
                let resp = req
                    .send_string(&body)
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                let status: u16 = resp.status();
                let body = resp
                    .into_string()
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                if body.len() > http_for_post.max_response_size {
                    return Err(mlua::Error::external(format!(
                        "Response too large: {} bytes (max {})",
                        body.len(),
                        http_for_post.max_response_size
                    )));
                }

                let result = lua.create_table()?;
                result.set("status", status as i64)?;
                result.set("body", body)?;
                Ok(result)
            },
        )?,
    )?;

    Ok(m)
}
