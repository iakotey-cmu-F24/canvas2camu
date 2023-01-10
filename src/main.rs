//! `Canvas2Camu` is a rust program to generate CAMU bulk-upload-compatible
//! excel files from Canvas' gradebook.



mod cli;
mod config;
mod enrollment;
mod gradebook;
mod utils;
mod writer;
use cli::parse_args;
use enrollment::parse_enrollment;
use gradebook::parse_gradebook_file;
use writer::create_files;
fn main() {

    let args = parse_args();
    
    let enrollment_data = parse_enrollment(&args.enrollment).unwrap();
    let gradebook = parse_gradebook_file(&args.gradebook).unwrap();

    create_files(&args.output_dir.to_str().unwrap(), &gradebook, &enrollment_data);
}
