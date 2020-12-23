extern crate clap;

use std::env::Args;
use std::fmt::Debug;

use clap::{App, AppSettings, Arg, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum Command {
    Compile,
    Check,
    Generate,
    Build,
    Test,
    Release,
    Rebuild,
    Clean,
    None,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ProjectType {
    App,
    Service,
    Lib,
    Ui,
}

#[derive(Debug)]
pub struct UserGoal {
    pub arg_command: Command,
    pub arg_file: Option<String>,
    pub arg_project_name: Option<String>,
    pub arg_project_type: ProjectType,
    pub arg_build_plan: Option<String>,
    pub arg_configuration: Option<String>,
}

pub fn parse(args: Args) -> Box<UserGoal> {
    let parsed_args = App::new("Dog")
        .version(VERSION)
        .about("A programming language for people who work with data.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("clean")
            .about("cleans output folder"))
        .subcommand(SubCommand::with_name("compile")
            .about("compiles a single file")
            .arg(Arg::with_name("file").required(true)))
        .subcommand(SubCommand::with_name("check")
            .about("syntax check on a single file")
            .arg(Arg::with_name("file").required(true)))
        .subcommand(SubCommand::with_name("format")
            .about("format a single file")
            .arg(Arg::with_name("file").required(true)))
        .subcommand(SubCommand::with_name("build")
            .about("builds a dog project")
            .arg(Arg::with_name("build-plan")
                .index(1)
                .help("specifies the file that defines the build plan."))
            .arg(Arg::with_name("configuration")
                .index(2)
                .help("specifies while configuration to use within the build plan.")))
        .subcommand(SubCommand::with_name("rebuild")
            .about("cleans and builds a dog project")
            .arg(Arg::with_name("build-plan")
                .index(1)
                .help("specifies the file that defines the build plan."))
            .arg(Arg::with_name("configuration")
                .index(2)
                .help("specifies while configuration to use within the build plan.")))
        .subcommand(SubCommand::with_name("release")
            .about("cleans and builds a dog project")
            .arg(Arg::with_name("build-plan")
                .index(1)
                .help("specifies the file that defines the build plan."))
            .arg(Arg::with_name("configuration")
                .index(2)
                .help("specifies while configuration to use within the build plan.")))
        .subcommand(SubCommand::with_name("test")
            .about("runs tests for a dog project")
            .arg(Arg::with_name("build-plan")
                .index(1)
                .help("specifies the file that defines the build plan."))
            .arg(Arg::with_name("configuration")
                .index(2)
                .help("specifies while configuration to use within the build plan.")))
        .subcommand(SubCommand::with_name("generate")
            .about("generates a dog project")
            .arg(Arg::with_name("project-name")
                .index(1)
                .required(true)
                .default_value("app")
                .help("name of the new project"))
            .arg(Arg::with_name("project-type")
                .index(2)
                .possible_values(&["app", "service", "lib", "ui"])
                .help("specifies the type of project.")))
        .get_matches_from(args);

    let mut result = UserGoal {
        arg_command: Command::None,
        arg_file: None,
        arg_project_name: None,
        arg_project_type: ProjectType::App,
        arg_build_plan: None,
        arg_configuration: None,
    };

    if let Some(sub_args) = parsed_args.subcommand_matches("compile") {
        let compile_file = sub_args.value_of("file").unwrap_or_default();
        result.arg_command = Command::Compile;
        result.arg_file = Some(String::from(compile_file));
        println!("Compiling {}", compile_file);
    } else if let Some(sub_args) = parsed_args.subcommand_matches("check") {
        let compile_file = sub_args.value_of("file").unwrap_or_default();
        result.arg_command = Command::Check;
        result.arg_file = Some(String::from(compile_file));
        println!("Checking {}", compile_file);
    } else if let Some(_sub_args) = parsed_args.subcommand_matches("clean") {
        result.arg_command = Command::Clean;
        println!("Cleaning");
    } else if let Some(sub_args) = parsed_args.subcommand_matches("build") {
        let build_plan = sub_args.value_of("build-plan").unwrap_or_default();
        let configuration = sub_args.value_of("configuration").unwrap_or_default();
        result.arg_command = Command::Build;
        result.arg_build_plan = Some(String::from(build_plan));
        result.arg_configuration = Some(String::from(configuration));
        println!("Building {} {}", build_plan, configuration);
    } else if let Some(sub_args) = parsed_args.subcommand_matches("rebuild") {
        let build_plan = sub_args.value_of("build-plan").unwrap_or_default();
        let configuration = sub_args.value_of("configuration").unwrap_or_default();
        result.arg_command = Command::Rebuild;
        result.arg_build_plan = Some(String::from(build_plan));
        result.arg_configuration = Some(String::from(configuration));
        println!("Rebuilding {} {}", build_plan, configuration);
    } else if let Some(sub_args) = parsed_args.subcommand_matches("test") {
        let build_plan = sub_args.value_of("build-plan").unwrap_or_default();
        let configuration = sub_args.value_of("configuration").unwrap_or_default();
        result.arg_command = Command::Test;
        result.arg_build_plan = Some(String::from(build_plan));
        result.arg_configuration = Some(String::from(configuration));
        println!("Testing {} {}", build_plan, configuration);
    } else if let Some(sub_args) = parsed_args.subcommand_matches("release") {
        let build_plan = sub_args.value_of("build-plan").unwrap_or_default();
        let configuration = sub_args.value_of("configuration").unwrap_or_default();
        result.arg_command = Command::Release;
        result.arg_build_plan = Some(String::from(build_plan));
        result.arg_configuration = Some(String::from(configuration));
        println!("Releasing {} {}", build_plan, configuration);
    } else if let Some(sub_args) = parsed_args.subcommand_matches("generate") {
        let project_name = sub_args.value_of("project-name").unwrap_or_default();
        let project_type = sub_args.value_of("project-type").unwrap_or_default();
        result.arg_command = Command::Generate;
        result.arg_project_name = Some(String::from(project_name));
        match project_type {
            "app" => result.arg_project_type = ProjectType::App,
            "lib" => result.arg_project_type = ProjectType::Lib,
            "service" => result.arg_project_type = ProjectType::Service,
            "ui" => result.arg_project_type = ProjectType::Ui,
            _ => result.arg_project_type = ProjectType::App
        }

        println!("Generating {} {:?}", project_name, result.arg_project_type);
    }

    return Box::new(result);
}