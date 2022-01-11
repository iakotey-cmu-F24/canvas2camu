/// https://stackoverflow.com/a/69324393
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

macro_rules! with_temp_dir{
 // macth like arm for macro
    ($path:expr, $code:expr)=>{

        {
            use std::env::{current_dir, set_current_dir};
            
            let old_path = current_dir().unwrap();
            
            set_current_dir($path).expect(&format!("Failed to set current directory to {}", $path));
            
            $code
            
            set_current_dir(&old_path).expect(&format!("Failed to set current directory to {:#?}", &old_path));
        }
    }
}

pub(crate) use cast;
pub(crate) use with_temp_dir;