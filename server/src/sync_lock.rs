use std::sync::Arc;

use dashmap::DashSet;

#[derive(Clone, Default)]
pub struct SyncLock {
    locks: Arc<DashSet<String>>,
}

pub struct LockHandle {}
