use dotenv::dotenv;
use std::io::Write;
use std::process;
extern crate customer_writer;
use structopt::StructOpt;

fn main() {
    dotenv().ok();

    let config = customer_writer::options::Opts::from_args();

    if let Err(ref e) = customer_writer::run(config) {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        process::exit(1);
    };
}

