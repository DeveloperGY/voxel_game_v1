use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    tx: Option<Sender<Box<dyn FnOnce() + Send>>>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        let mut threads = Vec::with_capacity(thread_count);
        let (tx, recv) = channel();
        let tx = Some(tx);
        let recv = Arc::new(Mutex::new(recv));

        for _ in 0..thread_count {
            let recv = Arc::clone(&recv);
            let thd = thread::spawn(move || {
                handle_jobs(recv);
            });
            threads.push(Some(thd));
        }

        Self { threads, tx }
    }

    pub fn run<J: FnOnce() + Send + 'static>(&self, job: J) {
        if let Some(tx) = self.tx.as_ref() {
            let _ = tx.send(Box::new(job));
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // This stops the threads if they are still running once they finish their jobs
        if let Some(tx) = self.tx.take() {
            drop(tx);
        }

        self.threads
            .iter_mut()
            .flat_map(Option::take)
            .for_each(|thd| {
                let _ = thd.join();
            });
    }
}

fn handle_jobs(recv: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send>>>>) {
    'recv: loop {
        // Scope is to ensure the lock is dropped before running the job
        let job = {
            let lock = if let Ok(lock) = recv.lock() {
                lock
            } else {
                break 'recv;
            };
            if let Ok(job) = lock.recv() {
                job
            } else {
                break 'recv;
            }
        };

        job();
    }
}
