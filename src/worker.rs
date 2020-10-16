use std::thread::sleep;
use std::time::{Duration, Instant};
use std::sync::atomic::{Ordering, AtomicBool};

use std::thread;
use std::thread::JoinHandle;
use crate::customer::{Customers, Customer, CustomersRetryWrapper};
use crate::SUCC_QUERIES;
use crate::ERR_QUERIES;
use std::sync::Arc;
use crate::repository::CustomerRepository;

pub struct WorkerPool {
    pub workers: Vec<Worker>,
}

// Automatically terminate threads when WorkerPool gets dropped.
impl Drop for WorkerPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("Number of successful queries: {}", SUCC_QUERIES.load(Ordering::SeqCst));
        println!("Number of errored queries: {}", ERR_QUERIES.load(Ordering::SeqCst));
    }
}

impl WorkerPool {
    pub fn new(thread_count: u32, max_qps: u32, should_work: Arc<AtomicBool>) -> WorkerPool {
        let mut workers = Vec::new();
        let qps_per_thread = max_qps as f32 / thread_count as f32;
        let sleep_time = Duration::from_millis((1000.0 / qps_per_thread) as u64);

        for _ in 0..thread_count {
            let offset = sleep_time / thread_count;
            sleep(offset);

            workers.push(Worker::new(sleep_time, should_work.clone()));
        }

        WorkerPool { workers }
    }
}

pub struct Worker {
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(sleep_time: Duration, should_work: Arc<AtomicBool>) -> Worker {
        let mut repo = CustomersRetryWrapper::new(Customers::new());

        let thread = thread::spawn(move || loop {
            if !should_work.load(Ordering::Relaxed) {
                return;
            }

            let customer = Customer::random();
            let start = Instant::now();

            // Keep logging for testing, continue anyway.
            if let Err(e) = repo.insert(&customer) {
                println!("Received error: {:?}", e);
                // Increment errored query counter.
                ERR_QUERIES.fetch_add(1,Ordering::SeqCst);
                continue;
            }

            println!(
                "Inserted customer with name: {}. Response Time: {:.2} seconds",
                &customer.name,
                start.elapsed().as_secs_f64()
            );

            // Increment successful query counter.
            SUCC_QUERIES.fetch_add(1,Ordering::SeqCst);

            // Account for network latency and do true throttling.
            let duration = start.elapsed();
            if duration < sleep_time {
                sleep(sleep_time - duration)
            } // TODO: Make throttling handle catch up here.
        });

        Worker {
            thread: Some(thread),
        }
    }
}

