// use std::sync::atomic::{AtomicUsize, Ordering};

// static COUNTER: AtomicUsize = AtomicUsize::new(0);

// #[inline]
// pub fn next_id() -> usize {
//     COUNTER.fetch_add(1, Ordering::Relaxed)
// }

static mut COUNTER: usize = 0;

#[inline(always)]
pub fn next_id() -> usize {
    let cur = unsafe { COUNTER };

    unsafe { COUNTER += 1 }

    cur
}