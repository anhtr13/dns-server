pub mod answer;
pub mod header;
pub mod question;
pub mod rfc;

use std::io::Cursor;

use anyhow::{Context, Result};

use crate::message::{answer::Answer, header::Header, question::Question};

#[derive(Clone)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(buf);
        let header = Header::parse(&mut reader);
        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            questions.push(Question::parse(&mut reader).context("Failed to parse question")?);
        }
        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            answers.push(Answer::parse(&mut reader).context("Failed to parse answer")?);
        }
        Ok(Self {
            header,
            questions,
            answers,
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.header.into_bytes());
        for q in self.questions {
            bytes.extend(q.into_bytes());
        }
        for a in self.answers {
            bytes.extend(a.into_bytes());
        }
        bytes
    }
}
