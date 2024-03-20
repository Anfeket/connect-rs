use std::net::TcpStream;

use message::{send_message, Message, TextMessage};

mod message;

fn main() {
    let mut connection = TcpStream::connect("127.0.0.1:6969").unwrap();
    let msg = TextMessage::msg("Hello World!".to_string());
    send_message(&msg, &mut connection).unwrap();
    send_message(&Message::Shutdown, &mut connection).unwrap();
}
