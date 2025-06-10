use core::ptr::NonNull;
use core::sync::atomic::{AtomicPtr};
use state_atomics::State;

mod state_atomics;

pub(crate) struct TaskHeader {
    pub(crate) state: State,
}

pub(crate) struct TaskRef {
    ptr: NonNull<TaskHeader>
}
unsafe impl Send for TaskRef where &'static TaskHeader: Send {}
unsafe impl Sync for TaskRef where &'static TaskHeader: Sync {}
impl TaskRef {
    
}

pub(crate) struct RunQueue {
    head: AtomicPtr<TaskHeader>,
}

pub(crate) struct SyncExecutor {
    run_queue: RunQueue,
    pender: Pender,
}

#[derive(Clone, Copy)]
pub(crate) struct Pender(*mut ());