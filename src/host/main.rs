use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data: Vec<u8>     = fs::read(&args[1]).unwrap();
    println!("contents : {:?}", data);
}
