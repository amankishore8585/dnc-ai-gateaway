use std::sync::atomic::{AtomicUsize, Ordering};

pub static REQUESTS_TOTAL: AtomicUsize = AtomicUsize::new(0);
pub static AUTH_FAILURES: AtomicUsize = AtomicUsize::new(0);
pub static RATE_LIMITED: AtomicUsize = AtomicUsize::new(0);
pub static SUCCESSFUL_REQUESTS: AtomicUsize = AtomicUsize::new(0); // NEW


pub fn metrics_text() -> String {
    format!(
        "requests_total {}\nauth_failures {}\nrate_limited {}\nsuccessful_requests {}\n",
        REQUESTS_TOTAL.load(Ordering::Relaxed),
        AUTH_FAILURES.load(Ordering::Relaxed),
        RATE_LIMITED.load(Ordering::Relaxed),
        SUCCESSFUL_REQUESTS.load(Ordering::Relaxed),   // NEW
    )
}