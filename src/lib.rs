#[macro_use]
extern crate lazy_static;

use crate::options::Opts;
use crate::worker::WorkerPool;
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::sync::Arc;


lazy_static! {
    static ref GLOBAL_COUNTER: AtomicU32 = AtomicU32::new(1000);
    static ref SUCC_QUERIES: AtomicU32 = AtomicU32::new(0);
    static ref ERR_QUERIES: AtomicU32 = AtomicU32::new(0);
    static ref CLIENT_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

pub mod worker;
pub mod customer;
pub mod options;
pub mod repository;

pub fn run(config: Opts) -> Result<(), mysql::Error> {
    let start = Instant::now();
    let should_work = Arc::new(AtomicBool::new(true));

    let worker_pool = WorkerPool::new(config.thread_count, config.max_qps, should_work.clone());

    while start.elapsed().as_secs() < config.run_time {
        sleep(Duration::new(0,100000000))
    }

    // Send the kill signal once we've run for as long as specified.
    should_work.store(false, Ordering::Relaxed);

    drop(worker_pool);
    Ok(())
}