mod message;

use std::net::UdpSocket;

use anyhow::Result;

use crate::message::{Message, answer::Answer};

fn main() -> Result<()> {
    println!("Logs from program:");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0u8; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut message = Message::from_bytes(&buf[..size])?;
                message.header.qr = 1;
                message.header.aa = 0;
                message.header.tc = 0;
                message.header.ra = 0;
                message.header.z = 0;
                message.header.rcode = if message.header.opcode == 0 { 0 } else { 4 };
                message.header.ancount = message.header.qdcount;

                for q in message.questions.iter() {
                    let mut a = Answer::default();
                    a.labels = q.labels.clone();
                    a.atype = q.qtype;
                    a.aclass = q.qclass;
                    a.ttl = 60;
                    a.length = 4;
                    a.data = vec![8, 8, 8, 8];
                    message.answers.push(a);
                }

                udp_socket
                    .send_to(&message.into_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break Err(e.into());
            }
        }
    }
}
