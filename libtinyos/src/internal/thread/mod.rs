use core::{
    cell::UnsafeCell,
    fmt::Debug,
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::null,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use alloc::{boxed::Box, sync::Arc};
use tinyos_abi::{
    flags::{TaskWaitOptions, WaitOptions},
    types::SysRetCode,
};

use crate::syscalls::{self, thread_exit};

pub type ThreadID = u64;
pub type TaskID = u64;

pub fn spawn<T, F>(f: F) -> ThreadingResult<JoinHandle<T>>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let my_packet: Arc<_> = JoinInner { ret: None.into() }.into();

    let their_packet = my_packet.clone();

    let main = move || {
        let result = f();
        unsafe { their_packet.ret.as_mut_unchecked() }.replace(result);
        unsafe { thread_exit() }
    };

    let main = Box::new(main);
    let main = Box::into_raw(main);

    let id = unsafe { syscalls::thread_create(main as *const (), null()) }.map_err(|_| {
        _ = unsafe { Box::from_raw(main) };
        ThreadingErr::Fail
    })?;

    Ok(JoinHandle {
        id: id,
        callback: main,
        inner: my_packet,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadingErr {
    Fail,
}

pub type ThreadingResult<T> = Result<T, ThreadingErr>;

struct JoinInner<T> {
    ret: UnsafeCell<Option<T>>,
}

impl<T> JoinInner<T> {
    fn receive(&self) -> ThreadingResult<T> {
        unsafe { self.ret.as_mut_unchecked() }
            .take()
            .ok_or(ThreadingErr::Fail)
    }
}

impl<T> Debug for JoinInner<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "ThreadInner {{ ... }}")
    }
}

#[derive(Debug)]
pub struct JoinHandle<T> {
    id: ThreadID,
    inner: Arc<JoinInner<T>>,
    callback: *mut dyn FnOnce(),
}

impl<T> JoinHandle<T> {
    pub fn get_id(&self) -> &ThreadID {
        &self.id
    }

    pub fn wait(&self, timeout: Duration) -> ThreadingResult<Option<T>> {
        self.wait_for(timeout.as_millis() as i64)
    }

    pub fn join(&self) -> ThreadingResult<T> {
        self.wait_for(-1)
            .map(|r| r.ok_or(ThreadingErr::Fail))
            .flatten()
    }

    fn wait_for(&self, timeout: i64) -> ThreadingResult<Option<T>> {
        _ = unsafe {
            syscalls::thread_join(
                self.id,
                timeout,
                WaitOptions::empty(),
                TaskWaitOptions::W_EXIT,
            )
        }
        .map_err(|_| ThreadingErr::Fail)?;
        self.inner.receive().map(|r| Some(r))
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        _ = unsafe { Box::from_raw(self.callback) };
    }
}
