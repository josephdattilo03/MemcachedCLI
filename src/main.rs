use clap::Parser;
use parsers::{Command, RetrievalCommand, StorageCommand};
use responses::{RetrievalResponse, StorageResponse};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
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

fn serialize_command(cmd_args: Vec<String>) -> (Result<String, String>, Command) {
    match cmd_args[0].as_str() {
        "set" | "add" | "replace" | "append" | "prepend" => {
            let memcached_command = match StorageCommand::parse(cmd_args) {
                Ok(command) => Ok(command.deserialize()),
                Err(string) => Err(string),
            };
            (memcached_command, Command::Storage)
        }
        "get" | "gets" | "gat" | "gats" => {
            let memcached_command = match RetrievalCommand::parse(cmd_args) {
                Ok(command) => Ok(command.deserialize()),
                Err(string) => Err(string),
            };
            (memcached_command, Command::Retrieval)
        }
        _ => (
            Err(format!("{} - command does not exist", cmd_args[0])),
            Command::Unknown,
        ),
    }
}

fn send_data(stream: &mut TcpStream, command: String) -> Result<String, String> {
    stream
        .write_all(command.as_bytes())
        .map_err(|_| "unable to write to stream".to_string())?;

    stream
        .flush()
        .map_err(|_| "buffer stream was unable to flush".to_string())?;

    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|_| "failed to set stream timeout".to_string())?;

    let mut buffer = Vec::new(); // Adjust the capacity based on expected data size

    loop {
        let mut temp_buffer = [0; 512]; // Adjust the size based on expected chunk size
        match stream.read(&mut temp_buffer) {
            Ok(0) => break, // EOF reached
            Ok(n) => buffer.extend_from_slice(&temp_buffer[..n]),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break, // Timeout
            Err(_) => return Err("failed to read return value from stream".to_string()),
        }
    }

    match String::from_utf8(buffer) {
        Ok(str) => Ok(str),
        Err(_) => Err("failed to convert stream from utf8".to_string()),
    }
}

fn serialize_response(response: String, cmd_type: Command) -> Option<String> {
    match cmd_type {
        Command::Storage => {
            let response = StorageResponse::serialize(response.as_str());
            response.get_message()
        }
        Command::Retrieval => {
            let response = RetrievalResponse::serialize(response.as_str());
            response.get_message()
        }
        Command::Unknown => None,
    }
}

fn start_service(stream: &mut TcpStream) {
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
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", cli.host, cli.port)) {
        if cli.command.len() == 0 {
            start_service(&mut stream);
        } else {
            let serialized_command = match serialize_command(cli.command) {
                (Ok(command), cmd_type) => (command, cmd_type),
                (Err(string), _) => {
                    println!("{}", string);
                    return;
                }
            };
            let response = {
                match send_data(&mut stream, serialized_command.0) {
                    Ok(res) => res,
                    Err(string) => {
                        println!("{}", string);
                        return;
                    }
                }
            };
            let serialized_response = serialize_response(response, serialized_command.1);
            if let Some(string) = serialized_response {
                println!("{}", string);
            }
        }
    } else {
        println!("failed to connect to memcached service");
    }
}
