use std::{future::Future, task::{Context, Waker, Poll, Wake}, sync::{Mutex, Condvar, Arc}, cell::RefCell, collections::VecDeque, marker::PhantomData};
use async_channel;
use futures::{future::BoxFuture, FutureExt};
use scoped_tls::scoped_thread_local;

scoped_thread_local!(static SIGNAL: Arc<Signal>);
// scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
scoped_thread_local!(static EX: Executor);

pub struct TaskQueue {
    queue: RefCell<VecDeque<Arc<Task>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue { queue: (RefCell::new(VecDeque::with_capacity(1024))) }
    }
    pub(crate) fn push(&self, runnable: Arc<Task>) {
        self.queue.borrow_mut().push_back(runnable);
    }
    pub fn pop(&self) -> Option<Arc<Task>> {
        self.queue.borrow_mut().pop_front()
    }
}

pub struct Executor {
    local_queue: TaskQueue,
    _marker: PhantomData<Arc<()>>,
}

impl Executor {
    pub fn new() -> Self {
        Self { 
            local_queue: TaskQueue::new(), 
            _marker: PhantomData, 
        }
    }

    pub fn spawn(fut: impl Future<Output = ()> + 'static + std::marker::Send) {
        let t = Arc::new(Task {
            future: RefCell::new(fut.boxed()),
            signal: Arc::new(Signal::new())
        });
        EX.with(|ex| ex.local_queue.push(t));
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(signal.clone());
        let mut cx = Context::from_waker(&waker);
        // let runnable = Mutex::new(VecDeque::<Arc<Task>>::with_capacity(1024));
        SIGNAL.set(&signal, || {
            EX.set(self, || {
            let mut fut= std::pin::pin!(future);
            loop {
                    if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                        return output;
                    }
                    while let Some(task) = self.local_queue.pop() {
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    }
                    if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                        return output;
                    }
                    signal.wait();
                }
            })
        })
    }
}

struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Waiting,
    Notified,
}

impl Signal {
    fn new() -> Self {
        Signal { state: Mutex::new(State::Empty), cond: Condvar::new() }
    }

    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("Mutiple wait!");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }

    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        EX.with(|ex| ex.local_queue.push(self.clone()));
        self.signal.notify();
    }
}

// struct Demo;

// impl Future for Demo {
//     type Output = ();

//     fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         println!("hello world!");
//         std::task::Poll::Ready(())
//     }
// }


// fn dummy_waker() -> Waker {
//     static DATA: () = ();
//     unsafe { Waker::from_raw(RawWaker::new(&DATA, &VTABLE)) }
// }

// const VTABLE: RawWakerVTable =
//     RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);

// unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
//     RawWaker::new(_p, &VTABLE)
// }

// unsafe fn vtable_wake(_p: *const ()) {}

// unsafe fn vtable_wake_by_ref(_p: *const ()) {}

// unsafe fn vtable_drop(_p: *const ()) {}

// async fn demo() {
//     println!("Hello world!");
// }


async fn demo() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    // std::thread::spawn(move|| {
    //     std::thread::sleep(Duration::from_secs(5));
    //     tx.send_blocking(())
    // });
    Executor::spawn(demo1(tx1));
    Executor::spawn(demo2(tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
}

async fn demo1(tx: async_channel::Sender<()>) {
    println!("Hello world!");
    let v = ["one", "two", "three", "four", "five"];
    for e in v {
        println!("{}", e);
    }
    let mut s = 0;
    for _i in 1..10000 {
        for j in 1..10000 {
            s += j;
            s %= 10007;
        }
    }
    println!("sum = {}", s);
    std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = tx.send(()).await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("Hallo Welt!");
    let v = ["eins", "zwei", "drei", "vier", "fünf"];
    for e in v {
        println!("{}", e);
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = tx.send(()).await;
}

fn main() {
    let ex = Executor::new();
    ex.block_on(demo());
}