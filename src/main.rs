use dotenv::dotenv;
use std::io::Write;
use std::process;
extern crate transmission;
use structopt::StructOpt;

fn main() {
    dotenv().ok();

    let config = transmission::options::Opts::from_args();

    if let Err(ref e) = transmission::run(config) {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        process::exit(1);
    };
}

