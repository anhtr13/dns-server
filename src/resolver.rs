use std::net::UdpSocket;

use anyhow::{Context, Result};

use crate::message::Message;

pub struct Resolver<'a> {
    addr: &'a str,
    socket: UdpSocket,
}

impl<'a> Resolver<'a> {
    pub fn new(addr: &'a str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").context("Failed to bind to socket")?;
        Ok(Self { addr, socket })
    }

    pub fn resolve(&self, mut message: Message) -> Result<Message> {
        message.header.qdcount = 1;
        let mut answers = Vec::new();

        for q in message.questions.iter() {
            let req = Message {
                header: message.header.clone(),
                questions: vec![q.clone()],
                answers: Vec::new(),
            };
            self.socket
                .send_to(&req.into_bytes(), self.addr)
                .context("Failed to send message")?;

            let mut buf = [0u8; 512];
            let (size, _) = self
                .socket
                .recv_from(&mut buf)
                .context("Failed to receive from resolver")?;

            let res = Message::from_bytes(&buf[..size]).context("Failed to read response")?;
            answers.extend(res.answers);
        }

        message.header.qdcount = answers.len() as u16;
        message.answers = answers;

        Ok(message)
    }
}
