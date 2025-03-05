use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&mut self, runnable: F)
    where
        F: FnOnce() + Send + 'static,
    {
        match &self.sender {
            Some(sender) => sender.send(Box::new(runnable)).unwrap(),
            None => panic!("ThreadPool::execute called after ThreadPool::drop"),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers.drain(..) {
            worker.handle.join().unwrap();
        }
    }
}

struct Worker {
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = {
                receiver
                    .lock()
                    .expect("Couldn't acquire lock to the Receiver")
                    .recv()
                    .unwrap()
            };
            println!("Worker {id} is running the job.");
            job();
        });
        Worker { handle }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
