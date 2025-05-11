use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self},
};
/// Struct ThreadPool
///
/// Examples
///
/// ```rust
/// use hello_web_server::ThreadPool;
///
/// // Create a thread pool with 4 threads
/// let thread_pool = ThreadPool::new(4);
///
/// // Perform this math operation in a separate thread
/// if let Ok(tp) = &thread_pool {
///     tp.execute(|| { 2 + 2; } );
/// }
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// `PoolCreationError` if capacity is zero.
    pub fn new(capacity: usize) -> Result<ThreadPool, PoolCreationError> {
        if capacity > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(capacity);

            for id in 0..capacity {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool {
                workers,
                sender: Some(sender),
            })
        } else {
            Err(PoolCreationError)
        }
    }

    /// Sends the given function f to a thread in the thread pool to be executed
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
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
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
