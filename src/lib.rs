use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
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
    sender: Option<mpsc::Sender<Job>>,
}

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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// execute method will send a job from the threadpool [sending end]
    /// to the available worker [receiving end]
    pub fn execute<F>(&self, f: F)
    where
        // FnOnce: because the thread for running a request will ony execute that request's closure one time
        // we use the () after FnOnce because this FnOnce represents a closure that takes no parameters and doesn't return a value
        // Send: to transfer the closure from one thread to another
        // 'static: because we don't know how long the thread will take to excute
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop the send to close the channel so that the reciever will know that
        // there are no more messages
        // and the infinite loop waiting for incomming jobs will be invalid
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // make the thread complete the job before it's dropped
                thread.join().unwrap();
            }
        }
    }
}

/// Job type holds the closure we want to send to the worker
/// type alisa fpr a Box trait object that holds the type that excute function recieves
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Worker Struct is Responsible for Sending Code from the ThreadPool to a Thread
/// because thread::spawn wanna give the thread a code to execute as soon as the thread is created
/// but we want to create thread and make them wait for the code
///
/// each worker will store a single JoinHandle<()> instance
/// and has a method that will take a closure of code to run and send it to the alreading running thread for excecution
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // we first call lock on receiver to aquire the mutex
            // then we call unwrap to panic if failed to obtain the lock
            // which might fail if the thread holding the lock panicked rather than releasing the
            // lock
            // then call recv to receive a job from the channel
            // the call to recv blocks until a job becomes available
            let message = receiver
                .lock()
                .expect("mutex is in poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock")
                .recv();

            match message {
                // the channel is open and receiving messages
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                // the channel is closed by dropping the sender for whatever reason
                Err(_) => {
                    println!("Worker {id} disconnected; Shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
