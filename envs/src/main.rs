#[macro_use]
extern crate dotenv_codegen;

// extern crate dotenv;
// use dotenv::dotenv;
// use std::env;

fn main() {
    // dotenv().ok();
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    println!("NAME: {}", dotenv!("NAME"));
    println!("VERSION: {}", dotenv!("VERSION"));
    println!("VERSION_SHOW: {}", dotenv!("VERSION_SHOW"));
}
