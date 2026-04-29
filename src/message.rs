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

        let base_labels = self.questions[0].labels.clone();
        let mut base_offsets = vec![0; base_labels.len()];
        base_offsets[0] = bytes.len();
        for i in 0..base_labels.len() - 1 {
            base_offsets[i + 1] = base_offsets[i] + base_labels[i].len() + 1;
        }

        for (i, q) in self.questions.into_iter().enumerate() {
            if i > 0
                && let lcs = longest_common_suffix(&base_labels, &q.labels)
                && lcs > 0
            {
                let offset = base_offsets[base_offsets.len() - lcs];
                let retained_len = q.labels.len() - lcs;
                bytes.extend(q.compress(retained_len, offset as u16));
            } else {
                bytes.extend(q.into_bytes());
            }
        }

        for a in self.answers {
            bytes.extend(a.into_bytes());
        }

        bytes
    }
}

fn longest_common_suffix(a1: &[String], a2: &[String]) -> usize {
    let (l1, l2) = (a1.len() - 1, a2.len() - 1);
    for i in 0..=l1.min(l2) {
        if a1[l1 - i] != a2[l2 - i] {
            return i;
        }
    }
    l1.min(l2)
}
