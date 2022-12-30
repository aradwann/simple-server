use std::thread;

pub struct ThreadPool{
    workers: Vec<Worker>,
}

impl ThreadPool{
    /// Create new ThreadPool
    /// 
    /// The size is the number of workers in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize)-> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size{
            // create some workers and store them in a vector
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            
        }
}

struct Worker{
    /// a worker holds a thread and is responsbile of sending the closure to be run 
    /// by a running thread because threads doesn't implement the behaviour of waiting for
    /// a code to run
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker{
    fn new(id:usize) -> Self{
        let thread = thread::spawn(||{});
        Worker { id, thread }
    }
}
