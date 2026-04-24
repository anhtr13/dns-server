mod db;
mod message;
mod resolver;

use std::{
    env::{self},
    net::UdpSocket,
};

use anyhow::{Context, Result};

use crate::{
    db::DataBase,
    message::{Message, answer::Answer},
    resolver::Resolver,
};

fn main() -> Result<()> {
    let db = DataBase::new();
    println!("Logs from program:");

    let args: Vec<_> = env::args().collect();
    let resolver = args.get(2);

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").context("Failed to bind to address")?;
    let mut buf = [0u8; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut message =
                    Message::from_bytes(&buf[..size]).context("Failed to parse message")?;

                if let Some(addr) = resolver {
                    let resolver = Resolver::new(addr).context("Failed to create resolver")?;
                    message = resolver
                        .resolve(message)
                        .context("Failed to resolve message")?;
                } else {
                    for q in message.questions.iter() {
                        let mut a = Answer::default();
                        a.labels = q.labels.clone();
                        a.atype = q.qtype;
                        a.aclass = q.qclass;
                        a.ttl = 60;
                        a.data = db.get(&q.labels.join(".")).unwrap_or(vec![8, 8, 8, 8]);
                        a.length = a.data.len() as u16;
                        message.answers.push(a);
                    }
                }

                message.header.qr = 1;
                message.header.aa = 0;
                message.header.tc = 0;
                message.header.ra = 0;
                message.header.z = 0;
                message.header.rcode = if message.header.opcode == 0 { 0 } else { 4 };
                message.header.ancount = message.header.qdcount;

                udp_socket
                    .send_to(&message.into_bytes(), source)
                    .context("Failed to send response")?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break Err(e.into());
            }
        }
    }
}
