use std::thread::JoinHandle as StdJoinHandle;

pub trait WaitableHandle {
    fn wait(self);
}

impl WaitableHandle for StdJoinHandle<()> {
    #[allow(unused_must_use)]
    fn wait(self) {
        self.join();
    }
}

pub struct Handle<T: WaitableHandle, TX, RX> {
    wait_handle: T,
    pub tx: TX,
    pub rx: RX,
}

impl<T: WaitableHandle, TX, RX> Handle<T, TX, RX> {
    pub fn wait(self) {
        self.wait_handle.wait();
    }
}

impl<TX, RX> Handle<StdJoinHandle<()>, TX, RX> {
    pub fn into_std(self) -> StdJoinHandle<()> {
        self.wait_handle
    }
}
