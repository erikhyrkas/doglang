use check::Check;
use generate::Generate;

use crate::goal::{Command, UserGoal};

mod rebuild;
mod generate;
mod check;
mod compile;
mod clean;
mod build;

#[derive(Debug)]
pub struct BuildData {
    pub goal: Box<UserGoal>,
    // list of files
    // list of externals?
}

pub trait Target {
    fn execute(&self);
}

pub fn create_target(goal: Box<UserGoal>) -> Box<dyn Target> {
    let result: Box<dyn Target>;

    let build_data = BuildData {
        goal: Box::new(*goal)
    };
    match (*build_data.goal).arg_command {
        Command::Generate => {
            result = Box::new(Generate {
                build_data,
            });
        }
        Command::Check => {
            result = Box::new(Check {
                build_data,
            });
        }
        /*
        Command::Compile => {}
        Command::Build => {}
        Command::Test => {}
        Command::Release => {}
        Command::Rebuild => {}
        Command::Clean => {}
        Command::None => {}*/
        _ => {
            panic!("command not implemented");
        }
    }

    return result;
}
