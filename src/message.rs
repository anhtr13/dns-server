pub mod answer;
pub mod header;
pub mod question;
pub mod rfc;

use std::io::Cursor;

use anyhow::Result;

use crate::message::{answer::Answer, header::Header, question::Question};

pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let mut data = Cursor::new(buf);
        let header = Header::parse(&mut data);
        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            questions.push(Question::parse(&mut data)?);
        }
        Ok(Self {
            header,
            questions,
            answers: Vec::new(),
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.into_bytes());
        for q in self.questions {
            res.extend(q.into_bytes());
        }
        for a in self.answers {
            res.extend(a.into_bytes());
        }
        res
    }
}
