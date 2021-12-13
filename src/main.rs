mod cli;
mod enrollment;
mod config;
mod utils;
use cli::parse_args;
use enrollment::parse_enrollment;

fn main() {
    let args = parse_args();
    let enrollment_data = parse_enrollment(& args.enrollment);
}
