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

pub struct Handle<T: JoinableHandle, TX, RX> {
    pub join_handle: T,
    pub tx: std::sync::mpsc::Sender<TX>,
    pub rx: std::sync::mpsc::Receiver<RX>,
}

impl<T: JoinableHandle, TX, RX> Handle<T, TX, RX> {
    pub fn join(self) {
        self.join_handle.join();
    }
    pub fn send(&mut self, msg: TX) {
        self.tx.send(msg).unwrap();
    }
    pub fn recv(&mut self) -> RX {
        self.rx.recv().unwrap()
    }
    pub fn try_recv(&mut self) -> Result<RX, std::sync::mpsc::TryRecvError> {
        self.rx.try_recv()
    }
}
