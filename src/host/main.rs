use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("hosting : {0}", args[1]);
}
