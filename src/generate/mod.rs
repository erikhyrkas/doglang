// project generator
// Generates a simple project using sane defaults

use app_gen::generate_app;

use crate::goal::ProjectType;
use crate::target::{BuildData, Target};

mod app_gen;

#[derive(Debug)]
pub struct Generate {
    pub build_data: BuildData,
}

impl Target for Generate {
    fn execute(&self) {
        //println!("Generate... {:?}", self.build_data);
        let project_name: &str = &self.build_data.goal.arg_project_name.as_ref().expect("No project name");
        match self.build_data.goal.arg_project_type {
            ProjectType::App => {
                generate_app(project_name);
            }
            ProjectType::Service => {
                unimplemented!();
            }
            ProjectType::Lib => {
                unimplemented!();
            }
            ProjectType::Ui => {
                unimplemented!();
            }
        }
    }
}
