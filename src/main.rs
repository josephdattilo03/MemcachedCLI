use clap::Parser;
use std::io::{self, Write};
use std::net::TcpStream;
mod parsers;
mod responses;

#[derive(Parser)]
#[command(version, about, long_about=None)]

struct Cli {
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,
    #[arg(long, short, default_value_t = 11211)]
    port: u16,
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn start_service(stream: TcpStream) {
    loop {
        print!("memcached ~ ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read command input");
    }
}

fn main() {
    let cli = Cli::parse();
    if let Ok(stream) = TcpStream::connect(format!("{}:{}", cli.host, cli.port)) {
        if cli.command.len() == 0 {
            start_service(stream);
        }
    } else {
        println!("failed to connect to memcached service");
    }
}
