#![feature(build_hasher_simple_hash_one)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}