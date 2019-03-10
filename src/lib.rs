use std::thread::JoinHandle as StdJoinHandle;

pub trait JoinableHandle {
    fn join(self);
}

impl JoinableHandle for StdJoinHandle<()> {
    #[allow(unused_must_use)]
    fn join(self) {
        self.join();
    }
}

pub struct Handle<T: JoinableHandle, TX> {
    pub join_handle: T,
    pub tx: std::sync::mpsc::Sender<TX>,
}

impl<T: JoinableHandle, TX> Handle<T, TX> {
    pub fn join(self) {
        self.join_handle.join();
    }
    pub fn send(&mut self, msg: TX) {
        self.tx.send(msg).unwrap();
    }
}
