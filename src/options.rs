extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "customer-writer",
    about = "Writes fake customers to a customer datterbase",
)]
/// You can use customer-writer to add some fake customers to a vitess backed customer datterbase.
pub struct Opts {
    /// Provide a number of threads to run with.
    #[structopt(short = "t", long = "threads", default_value = "32")]
    pub(crate) thread_count: u32,

    /// Provide a max QPS value for throttling.
    #[structopt(short = "m", long = "max_qps", default_value = "5000")]
    pub(crate) max_qps: u32,

    /// Provide a max run time.
    #[structopt(short = "r", long = "run_time", default_value = "10")]
    pub(crate) run_time: u64,
}
