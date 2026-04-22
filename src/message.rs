pub mod answer;
pub mod header;
pub mod question;
pub mod rfc;

use std::io::Cursor;

use anyhow::Result;

use crate::message::{answer::Answer, header::Header, question::Question};

pub struct Message {
    pub header: Header,
    pub question: Question,
    pub answer: Answer,
}

impl Message {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let mut data = Cursor::new(buf);
        let header = Header::parse(&mut data);
        let question = Question::parse(&mut data)?;
        Ok(Self {
            header,
            question,
            answer: Answer::default(),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.serialize());
        res.extend(self.question.serialize());
        res.extend(self.answer.serialize());
        res
    }
}
