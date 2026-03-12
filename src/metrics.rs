use std::sync::atomic::{AtomicUsize, Ordering};

pub static REQUESTS_TOTAL: AtomicUsize = AtomicUsize::new(0);
pub static AUTH_FAILURES: AtomicUsize = AtomicUsize::new(0);
pub static RATE_LIMITED: AtomicUsize = AtomicUsize::new(0);

pub fn metrics_text() -> String {
    format!(
        "requests_total {}\nauth_failures {}\nrate_limited {}\n",
        REQUESTS_TOTAL.load(Ordering::Relaxed),
        AUTH_FAILURES.load(Ordering::Relaxed),
        RATE_LIMITED.load(Ordering::Relaxed),
    )
}