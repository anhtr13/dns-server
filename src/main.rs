mod message;

use std::net::UdpSocket;

use anyhow::Result;

use crate::message::{Header, Message, Question};

fn main() -> Result<()> {
    println!("Logs from program:");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let question = Question::new("codecrafters.io".to_string(), 1, 1)?;
                let header = Header {
                    qdcount: 1,
                    ..Default::default()
                };
                let message = Message::new(header, question);
                udp_socket
                    .send_to(&message.serialize(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break Ok(());
            }
        }
    }
}
