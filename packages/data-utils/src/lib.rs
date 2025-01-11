

pub mod check;
pub mod dump;
pub mod readtest;

mod error;
pub use error::Error;

pub fn thread_pool() -> threadpool::ThreadPool {
    // not subtracting 1 since the work on main thread is minimal
    let num_cpus = num_cpus::get().max(1);
    threadpool::ThreadPool::new(num_cpus)
}

mod util;