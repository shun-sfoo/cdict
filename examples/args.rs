use std::env::args;

fn main() {
    let args: Vec<String> = args().into_iter().collect();

    println!("{:?}", args);
}
