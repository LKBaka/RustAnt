use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}