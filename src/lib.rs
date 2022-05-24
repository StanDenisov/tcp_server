use std::fmt::Display;


use std::sync::{mpsc, Arc, Mutex};
use std::{fmt, thread};

pub struct PoolCreationError {
    v: String,
}

impl PoolCreationError {
    fn new() -> PoolCreationError {
        PoolCreationError {
            v: "oh no size is size < 0!".to_string(),
        }
    }
    fn change_message(&mut self, new_message: &str) {
        self.v = new_message.to_string();
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("v", &self.v)
         .finish()
    }
}
impl Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", &self.v)
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} recive the job", id);
            job.call_box();
            }
        });
        Worker { id, thread }
    }
}


trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F : FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
       (*self)(); 
    }
} 

type Job = Box<dyn FnBox + Send + 'static>;



pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel();
            let mut threads = Vec::with_capacity(size);
            let receiver = Arc::new(Mutex::new(receiver));
            for i in 0..size {
                threads.push(Worker::new(i, Arc::clone(&receiver)))
            }
            ThreadPool { threads, sender }
        }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
