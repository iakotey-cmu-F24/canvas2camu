mod cli;
mod config;
mod enrollment;
mod gradebook;
mod utils;
use cli::parse_args;
use enrollment::parse_enrollment;
use gradebook::parse_gradebook_file;

fn main() {
    let args = parse_args();
    let enrollment_data = parse_enrollment(&args.enrollment);
    let gradebook = parse_gradebook_file(&args.gradebook);
}
