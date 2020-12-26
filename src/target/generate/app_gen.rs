use std::fs;
use std::io::Write;
use std::path::Path;

const APP_CONFIG: &'static str = "
config Release {
    version: string = '1.0.0'
}
";

const APP_MAIN: &'static str = "
app HelloWorld() {
  println('Hello world!')
}
";

pub fn generate_app(name: &str) {
    println!("Creating application folder");
    fs::create_dir(name).expect("Failed to create project folder");

    println!("Creating application config");
    let full_config_path = Path::new(name).join("config.dog");
    let mut config_file = std::fs::File::create(full_config_path).expect("create failed");
    config_file.write_all(APP_CONFIG.as_bytes()).expect("write failed");

    println!("Creating application main");
    let full_main_path = Path::new(name).join("main.dog");
    let mut main_file = std::fs::File::create(full_main_path).expect("create failed");
    main_file.write_all(APP_MAIN.as_bytes()).expect("write failed");
    println!("Application {} created.", name);
}