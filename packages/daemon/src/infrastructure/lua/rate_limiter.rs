//! Rate limiter for Lua module calls.
//!
//! Tracks call counts per module within a sliding time window.
//! When the call count exceeds the configured limit, further calls are rejected
//! until the window expires.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for '{0}': {1} calls per {2:?}")]
    Exceeded(String, usize, Duration),
}

/// Per-module rate limit configuration: (max_calls, window_duration).
type ModuleLimit = (usize, Duration);

/// A generic rate limiter that tracks call counts per time window.
///
/// Clone is implemented via `Arc<Mutex<>>`, so multiple module creators
/// can share the same limiter instance.
#[derive(Clone)]
pub struct RateLimiter {
    /// Per-module call timestamps.
    calls: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    /// Max calls per window per module.
    limits: HashMap<String, ModuleLimit>,
}

impl RateLimiter {
    /// Create a new rate limiter with default limits for all waio modules.
    pub fn new() -> Self {
        let mut limits = HashMap::new();

        // Default limits.
        limits.insert("http".to_string(), (60, Duration::from_secs(60)));
        limits.insert("exec".to_string(), (10, Duration::from_secs(60)));
        limits.insert("fs_read".to_string(), (100, Duration::from_secs(60)));
        limits.insert("notify".to_string(), (30, Duration::from_secs(60)));
        limits.insert("system".to_string(), (20, Duration::from_secs(60)));
        limits.insert("network".to_string(), (30, Duration::from_secs(60)));

        Self {
            calls: Arc::new(Mutex::new(HashMap::new())),
            limits,
        }
    }

    /// Check if a call is allowed for the given module.
    /// Returns `Err(RateLimitError)` if the limit is exceeded.
    pub fn check(&self, module: &str) -> Result<(), RateLimitError> {
        let (limit, window) = match self.limits.get(module) {
            Some(l) => l,
            None => return Ok(()), // No limit configured for this module.
        };

        let now = Instant::now();
        let cutoff = now.checked_sub(*window).unwrap_or(now);

        let mut calls = self.calls.lock().expect("RateLimiter mutex poisoned");
        let timestamps = calls.entry(module.to_string()).or_default();

        // Prune old entries outside the window.
        timestamps.retain(|t| *t > cutoff);

        if timestamps.len() >= *limit {
            return Err(RateLimitError::Exceeded(
                module.to_string(),
                *limit,
                *window,
            ));
        }

        Ok(())
    }

    /// Record a call for the given module (prunes old entries).
    pub fn record(&self, module: &str) {
        let window = match self.limits.get(module) {
            Some((_, w)) => *w,
            None => return, // No limit configured.
        };

        let now = Instant::now();
        let cutoff = now.checked_sub(window).unwrap_or(now);

        let mut calls = self.calls.lock().expect("RateLimiter mutex poisoned");
        let timestamps = calls.entry(module.to_string()).or_default();

        // Prune old entries.
        timestamps.retain(|t| *t > cutoff);
        timestamps.push(now);
    }

    /// Check and record a call in one operation.
    /// Returns `Err(RateLimitError)` if the limit would be exceeded.
    pub fn check_and_record(&self, module: &str) -> Result<(), RateLimitError> {
        self.check(module)?;
        self.record(module);
        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits_exist() {
        let rl = RateLimiter::new();
        // Should not panic and should have defaults.
        assert!(rl.check("http").is_ok());
        assert!(rl.check("exec").is_ok());
        assert!(rl.check("fs_read").is_ok());
        assert!(rl.check("notify").is_ok());
        assert!(rl.check("system").is_ok());
        assert!(rl.check("network").is_ok());
    }

    #[test]
    fn test_unknown_module_always_ok() {
        let rl = RateLimiter::new();
        assert!(rl.check("unknown_module").is_ok());
    }

    #[test]
    fn test_rate_limit_enforced() {
        let mut rl = RateLimiter::new();
        // Override with a very low limit for testing.
        rl.limits
            .insert("test".to_string(), (2, Duration::from_secs(60)));

        assert!(rl.check_and_record("test").is_ok());
        assert!(rl.check_and_record("test").is_ok());
        // Third call should fail.
        assert!(rl.check_and_record("test").is_err());
    }

    #[test]
    fn test_clone_shares_state() {
        let rl = RateLimiter::new();
        let rl2 = rl.clone();

        // Both share the same inner state.
        rl.record("http");
        // rl2 should see the same counts.
        assert!(rl2.check("http").is_ok()); // 1 < 60
    }

    #[test]
    fn test_record_prunes_old_entries() {
        let mut rl = RateLimiter::new();
        // Use a zero-duration window to force all entries to be "old".
        rl.limits
            .insert("test".to_string(), (5, Duration::from_secs(0)));

        // Record a call (will be immediately pruned).
        rl.record("test");

        // Check should succeed since the entry was pruned.
        assert!(rl.check("test").is_ok());
    }
}
