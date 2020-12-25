use std::env;

mod goal;
mod target;
mod lex;
mod transform;
mod parse;
//mod llvm;

fn main() {
    let user_goal = goal::parse(env::args());
    let target = target::create_target(user_goal);
    target.execute();
}

const UNKNOWN: &str = "Unknown";
