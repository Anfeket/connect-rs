#![allow(dead_code)]
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub enum Message {
    Text(TextMessage),
    File(FileMessage),
    Command(CommandMessage),
    Goodbye,
    Shutdown,
}
impl Message {
    fn deserialize(data: &[u8]) -> Option<Self> {
        if let Some((byte, data)) = data.split_first() {
            return match byte {
                0 => Some(Message::Text(TextMessage::deserialize(data)?)),
                1 => Some(Message::File(FileMessage::deserialize(data)?)),
                2 => Some(Message::Command(CommandMessage::deserialize(data)?)),
                3 => Some(Message::Goodbye),
                4 => Some(Message::Shutdown),
                _ => None
            }
        };
        None
    }
    fn serialize(&self) -> Vec<u8> {
        match self {
            Message::Text(msg) => {
                let mut data = vec![0];
                data.extend(TextMessage::serialize(msg));
                data
            }
            Message::File(msg) => {
                let mut data = vec![1];
                data.extend(FileMessage::serialize(msg));
                data
            }
            Message::Command(msg) => {
                let mut data = vec![2];
                data.extend(CommandMessage::serialize(msg));
                data
            }
            Message::Goodbye => {
                vec![3]
            }
            Message::Shutdown => vec![4],
        }
    }
}
pub struct TextMessage {
    pub text: String,
}
pub struct FileMessage {
    pub file_name: String,
    pub data: Vec<u8>,
}
pub struct CommandMessage {
    pub command: String,
}

impl TextMessage {
    fn serialize(&self) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        Some(TextMessage {
            text: String::from_utf8_lossy(data).into(),
        })
    }

    pub fn msg(text: String) -> Message {
        Message::Text(TextMessage { text })
    }
}
impl FileMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        let file_name = self.file_name.as_bytes();
        let file_name_size = (file_name.len() as u64).to_be_bytes();
        data.extend(file_name_size);
        data.extend(file_name);

        let file_data_size = (self.data.len() as u64).to_be_bytes();
        data.extend(file_data_size);
        data.extend(&self.data);
        data
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut cursor = std::io::Cursor::new(data);

        let mut file_name_length = [0; 8];
        if cursor.read_exact(&mut file_name_length).is_err() {
            return None;
        }
        let file_name_length = u64::from_be_bytes(file_name_length);

        let mut file_name = vec![0; file_name_length as usize];
        if cursor.read_exact(&mut file_name).is_err() {
            return None;
        }
        let file_name = String::from_utf8_lossy(&file_name).into();

        let mut data = Vec::new();
        if cursor.read_to_end(&mut data).is_err() {
            return None;
        }

        Some(Self { file_name, data })
    }
}
impl CommandMessage {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(_data: &[u8]) -> Option<Self> {
        todo!()
    }
}

pub fn send_message(message: &Message, stream: &mut TcpStream) -> std::io::Result<()> {
    let data = message.serialize();
    let data_length = data.len() as u64;
    stream.write_all(&data_length.to_be_bytes())?;
    stream.write_all(&data)?;
    Ok(())
}
pub fn receive_message(stream: &mut TcpStream) -> std::io::Result<Option<Message>> {
    let mut length = [0; 8];
    stream.read_exact(&mut length)?;
    let length = u64::from_be_bytes(length) as usize;

    let mut message = vec![0; length];
    stream.read_exact(&mut message)?;
    let message = Message::deserialize(&message);
    Ok(message)
}
