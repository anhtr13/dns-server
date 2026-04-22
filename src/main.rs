mod message;

use std::net::UdpSocket;

use anyhow::Result;

use crate::message::Message;

fn main() -> Result<()> {
    println!("Logs from program:");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0u8; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut message = Message::parse(&buf[..size])?;
                message.header.qr = 1;
                message.header.aa = 0;
                message.header.tc = 0;
                message.header.ra = 0;
                message.header.z = 0;
                message.header.rcode = if message.header.opcode == 0 { 0 } else { 4 };
                message.header.ancount = 1;

                message.answer.name = message.question.name.clone();
                message.answer.atype = message.question.qtype;
                message.answer.aclass = message.question.qclass;
                message.answer.ttl = 60;
                message.answer.length = 4;
                message.answer.data = vec![8, 8, 8, 8];

                udp_socket
                    .send_to(&message.serialize(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break Err(e.into());
            }
        }
    }
}
