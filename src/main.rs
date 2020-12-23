use std::env;

mod goal;
mod target;
mod check;
mod generate;
mod lex;
mod transform;

fn main() {
    let user_goal = goal::parse(env::args());
    let target = target::create_target(user_goal);
    target.execute();
}
