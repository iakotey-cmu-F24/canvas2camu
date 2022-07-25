//! `Grade_Gen` is a rust program to generate CAMU bulk-upload-compatible
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
    let enrollment_data = parse_enrollment(&args.enrollment);
    let gradebook = parse_gradebook_file(&args.gradebook).unwrap();
    // println!("{:#?}", enrollment_data);
    // println!("{:#?}", gradebook);
    // println!("{:#?}", args);
    // create_files(r"D:\OneDrive\Projects\grade_gen\src\tests\Assets\tmp", &gradebook, &enrollment_data);
    create_files(&args.output_dir.to_str().unwrap(), &gradebook, &enrollment_data);
}
