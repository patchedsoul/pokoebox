use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    /// Counter used to obtain unique handle IDs.
    static ref SOURCE_HANDLE_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(usize);

impl Handle {
    /// Allocate a new unique source handle.
    pub fn unique() -> Self {
        Self(SOURCE_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteHandle {
    None,
    Mpris(crate::mpris::PlayerHandle),
}
