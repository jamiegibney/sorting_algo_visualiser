use super::*;
use std::{
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
pub struct Sleep {
    output_arr: Arc<Mutex<Vec<usize>>>,
}

impl Sleep {
    pub fn new() -> Self {
        Self { output_arr: Arc::new(Mutex::new(vec![])) }
    }
}

impl SortProcessor for Sleep {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        self.output_arr.lock().reserve_exact(n);

        let mut threads = vec![];

        for i in 0..n {
            let out = Arc::clone(&self.output_arr);
            let element = arr.read(i);

            threads.push(spawn(move || {
                thread_priority::set_current_thread_priority(
                    thread_priority::ThreadPriority::Max,
                );

                sleep(Duration::from_millis(element as u64 * 10));
                out.lock().push(element);
            }));
        }

        for th in threads {
            th.join();
        }

        let mut out = self.output_arr.lock();
        for i in 0..n {
            arr.write(i, out[i]);
        }

        out.clear();
    }
}
