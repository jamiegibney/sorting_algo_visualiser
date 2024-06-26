use std::{
    io::{Error, Result as IoResult},
    panic,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        mpsc, Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use PoolCreationError as PCE;

type ReceiverArc = Arc<Mutex<mpsc::Receiver<Job>>>;
type Job = Box<dyn FnMut() + Send + 'static>;

/// A general-purpose thread pool.
///
/// You can use this as a way of performing work asynchronously on however
/// many threads you need. See the
/// [`block_until_free()`](ThreadPool::block_until_free) method if you need to
/// wait for all jobs to be finished until continuing.
///
/// When calling the [`execute()`](ThreadPool::execute) method, the pool will
/// send the job down a channel where it is queued, and then the next thread
/// to try to receive from the channel will unwrap and process it.
///
/// It is possible to see the number of currently-queued jobs, or number of
/// idle worker threads, at any given time using the
/// [`queued_jobs()`](ThreadPool::queued_jobs)
/// and [`num_idle()`](ThreadPool::num_idle) methods.
///
/// The pool will automatically clean up and join all worker threads when it is
/// dropped.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    queue: Arc<AtomicUsize>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreads,
    FailedSpawn(Error),
}

#[derive(Debug)]
struct Worker {
    _id: usize,
    thread: Option<JoinHandle<()>>,
    is_idle: Arc<AtomicBool>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: ReceiverArc,
        queue: Arc<AtomicUsize>,
        priority: Option<thread_priority::ThreadPriority>,
        name: &str,
    ) -> IoResult<Self> {
        let builder = thread::Builder::new();

        let is_idle = Arc::new(AtomicBool::new(true));
        let is_idle_ref = Arc::clone(&is_idle);

        let thread = builder
            .name(format!("thread `{name}` (pool id {id})"))
            .spawn(move || loop {
                if let Some(priority) = priority {
                    _ = thread_priority::set_current_thread_priority(priority);
                }

                // set the idle state to true
                is_idle_ref.store(true, Relaxed);

                // then block and wait for a message (i.e. a task)
                let msg = receiver.lock().unwrap().recv();

                // when a task is received, decrement the queue counter
                queue.fetch_sub(1, Relaxed);
                // and set the worker thread as not idle
                is_idle_ref.store(false, Relaxed);

                match msg {
                    Ok(mut job) => {
                        let result = panic::catch_unwind(
                            panic::AssertUnwindSafe(|| {
                                job();
                            }),
                        );

                        if result.is_err() {
                            eprintln!(
                                "thread {:?} panicked!",
                                thread::current().name().unwrap_or("unnamed"),
                            );
                        }
                    }
                    Err(_) => break,
                }
            })?;

        Ok(Self { _id: id, thread: Some(thread), is_idle })
    }

    fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

impl ThreadPool {
    /// Builds a new `ThreadPool`.
    ///
    /// # Errors
    ///
    /// Returns a `PoolCreationError` if `num_threads == 0`, or if any of the
    /// requested threads failed to spawn.
    pub fn build(
        num_threads: usize,
        priority: Option<thread_priority::ThreadPriority>,
        names: Option<&[&str]>,
    ) -> Result<Self, PoolCreationError> {
        if num_threads == 0 {
            return Err(PCE::ZeroThreads);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        let queue = Arc::new(AtomicUsize::new(0));

        for id in 0..num_threads {
            let name = names.map_or("", |s| s[id]);
            match Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&queue),
                priority,
                name,
            ) {
                Ok(worker) => workers.push(worker),
                Err(e) => return Err(PCE::FailedSpawn(e)),
            }
        }

        Ok(Self { workers, sender: Some(sender), queue })
    }

    /// Sends a closure to the thread pool, which adds it to a queue where it
    /// may be processed by one of the worker threads.
    ///
    /// This function does not guarantee that the provided closure will be
    /// processed immediately.
    ///
    /// # See also
    /// [`wait_until_done()`](Self::wait_until_done) - use this method if you
    /// need to ensure that all worker threads finish the jobs you provide
    /// before continuing.
    #[allow(clippy::missing_panics_doc)]
    pub fn execute<F>(&self, f: F)
    where
        F: FnMut() + Send + 'static,
    {
        self.queue.fetch_add(1, Relaxed);
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    /// Blocks the calling thread until all worker threads are idle. Use this
    /// method if you need to ensure that all worker threads finish the jobs
    /// you have provided before continuing.
    pub fn block_until_free(&self) {
        while !self.is_idle() {}
    }

    /// Returns whether all of the `ThreadPool`'s worker threads are idle or
    /// not.
    pub fn is_idle(&self) -> bool {
        self.workers.iter().all(|w| w.is_idle.load(Relaxed))
            && self.queued_jobs() == 0
    }

    /// Returns the number of idle worker threads in the `ThreadPool`.
    pub fn num_idle(&self) -> usize {
        self.workers
            .iter()
            .filter(|w| w.is_idle.load(Relaxed))
            .count()
    }

    /// Returns the current number of queued jobs.
    pub fn queued_jobs(&self) -> usize {
        self.queue.load(Relaxed)
    }

    /// Returns the number of threads held in the pool.
    pub fn num_threads(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            worker.join();
        }
    }
}
