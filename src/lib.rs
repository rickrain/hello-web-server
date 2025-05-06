use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the capacity is zero.
    pub fn new(capacity: usize) -> ThreadPool {
        assert!(capacity > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity);

        for id in 0..capacity {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Creates a new ThreadPool
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// `PoolCreationError` if capacity is zero.
    ///
    /// The `new` function will panic if the capacity is zero.
    pub fn build(capacity: usize) -> Result<ThreadPool, PoolCreationError> {
        if capacity > 0 {
            Ok(Self::new(capacity))
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
