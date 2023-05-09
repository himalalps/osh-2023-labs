use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "The size of ThreadPool should be positive.");

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            threads.push(thread::spawn(|| {}));
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
