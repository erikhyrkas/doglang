use std::fs;
use std::path::Path;

use crate::lex::lex;
// file syntax checker
use crate::target::{BuildData, Target};

#[derive(Debug)]
pub struct Check {
    pub build_data: BuildData,
}

impl Target for Check {
    fn execute(&self) {
        let file_name: &str = &self.build_data.goal.arg_file.as_ref().expect("No file name");
        let file_text = Box::new(fs::read_to_string(file_name).unwrap());
        let path = fs::canonicalize(Path::new(file_name)).unwrap();
        let _token_stream = lex(file_text.as_ref(), Some(&file_name), path.to_str());
        println!("parsing...");
    }
}
