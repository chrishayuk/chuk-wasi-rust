use std::{env, fs};
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = args[1].clone();

    println!("Hello {}",name);

    let mut file = fs::File::create("hello-chuk.txt").unwrap();
    write!(file,"hello {}\n", name).unwrap();
}
