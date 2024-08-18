use std::env;

mod parsers;
mod responses;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host: String = String::from("localhost");
    let port: String = String::from("11211");
    for (idx, arg) in args.iter().enumerate() {
        println!("{}", arg);
    }
}
