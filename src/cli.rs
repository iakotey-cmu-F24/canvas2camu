use std::path::{PathBuf};

use argparse::{ArgumentParser, Store};
use dirs::home_dir;

///Struct to hold CLI arguments
#[derive(Debug)]
pub(crate) struct ArgStruct {
    pub(crate) enrollment: String,
    pub(crate) gradebook: String,
    pub(crate) output_dir: PathBuf,
}
impl ArgStruct {
    fn new() -> ArgStruct {
        ArgStruct {
            enrollment: String::new(),
            gradebook: String::new(),
            output_dir: PathBuf::new(),
        }
    }
}

pub(crate) fn parse_args() -> ArgStruct {
    let mut args = ArgStruct::new();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Grade generator for CAMU SIS");
        parser
            .refer(&mut args.enrollment)
            .add_argument(
                "enrollment",
                Store,
                "Path to enrollment data from CAMU",
            )
            .required();
        parser
            .refer(&mut args.gradebook)
            .add_argument("gradebook", Store, "Path to gradebook from Canvas")
            .required();
        parser.refer(&mut args.output_dir).add_option(
            &["-o", "--output"],
            Store,
            "Output directory",
        );

        parser.parse_args_or_exit();
    }
    {

        if !args.output_dir.is_dir() {
            match home_dir() {
                Some(path) =>{
                    args.output_dir = path;
                }
                None => {
                    panic!("{}", "Could not set output directory");
                }
                
            }
        }

    }
    println!("{:#?}", args);
    args
}
