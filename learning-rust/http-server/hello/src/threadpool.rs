use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let rc_receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rc_receiver)))
        }

        ThreadPool {
            _workers: workers,
            sender,
        }
    }

    pub fn execute<F>(&mut self, runnable: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(runnable)).unwrap()
    }
}

struct Worker {
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let lock = receiver
                .lock()
                .expect("Couldn't acquire lock to the Receiver");
            let job = lock.recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { _handle: handle }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
