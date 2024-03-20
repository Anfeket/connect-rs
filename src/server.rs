use std::{net::{TcpListener, TcpStream}, process::exit};

use message::{receive_message, Message};
mod message;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    loop {
        let msg = receive_message(&mut stream);
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        };
        let msg = match msg {
            Some(msg) => msg,
            None => {
                eprintln!("Failed to deserialize");
                break;
            }
        };
        match msg {
            Message::Text(message) => println!("{}", message.text),
            Message::File(file) => {
                println!("File: {}, {}KB", file.file_name, file.data.len() / 1024)
            }
            Message::Command(command) => println!("CMD: {}", command.command),
            Message::Goodbye => {
                println!("{} says goodbye!", stream.peer_addr().unwrap());
                break;
            }
            Message::Shutdown => {
                println!("Shutting down...");
                exit(0)
            },
        }
    }
}
