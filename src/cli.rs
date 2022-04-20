use std::path::PathBuf;

use argparse::{ArgumentParser, Store, StoreTrue};
use dirs::home_dir;

///Struct to hold CLI arguments
#[derive(Debug)]
pub(crate) struct ArgStruct {
    pub(crate) enrollment: String,
    pub(crate) gradebook: String,
    pub(crate) output_dir: PathBuf,
    pub(crate) as_zip: bool,
}

impl ArgStruct {
    fn new() -> ArgStruct {
        ArgStruct {
            enrollment: String::new(),
            gradebook: String::new(),
            output_dir: PathBuf::new(),
            as_zip: false,
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
        parser.refer(&mut args.as_zip).add_option(
            &["-z", "--zip"],
            StoreTrue,
            "Save as zip",
        );

        parser.parse_args_or_exit();
    }
    {
        if !args.output_dir.is_dir() {
            args.output_dir = home_dir().expect(
                "Output directory could not be found: Try using -o option",
            );
        };
    }
    println!("{:#?}", args);
    args
}
