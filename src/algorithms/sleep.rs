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

        let mut threads = vec![];

        for i in 0..n {
            let out = Arc::clone(&self.output_arr);
            let element = arr.read(i);

            threads.push(spawn(move || {
                sleep(Duration::from_millis(element as u64 * 10));
                out.lock().push(element);
            }));
        }

        for th in threads {
            th.join();
        }

        let out = self.output_arr.lock();
        for i in 0..n {
            arr.write(i, out[i]);
        }
        drop(out);

        self.output_arr.lock().clear();
    }
}
