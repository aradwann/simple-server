use std::{sync::{mpsc, Arc, Mutex}, thread};
/// ThreadPool is a struct that holds workers[waiting threads] that are waiting for jobs to do
///
/// its purpose: is to enable concurrent work through multi-threading
///
/// it has two private fields:
///
/// workers: is vector of workers [waiting threads]
///
/// sender: is the sender part of the channel that's used to communicate with threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create new ThreadPool
    ///
    /// The size is the number of workers in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        // we want to pass receiver to worker, yet we can't just clone it 
        // as it's thread unsafe (race conditions) and we don't want to send a message
        // multiple times, we want only one reveiver that's owned by the workers
        // we'll user Arc<> to share ownership across threads 
        // [let multiple workers own the receiver] 
        // Arc<Mutex> to allow threads to mutate the value
        // [ensure that only one worker get a job from the receiver at a time]
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some workers and store them in a vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

/// a worker holds a thread and is responsbile of sending the closure to be run
/// by a running thread because threads doesn't implement the behaviour of waiting for
/// a code to run as threaded are looking for a code to run as soons as it's created
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {

        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}
