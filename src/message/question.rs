use std::io::{Cursor, Read};

use anyhow::Result;
use bytes::Buf;

use crate::message::rfc::{RfcClass, RfcType};

pub struct Question {
    pub name: String,
    pub qtype: RfcType,
    pub qclass: RfcClass,
}

impl Question {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut name = String::new();
        while data.has_remaining() {
            let b = data.get_u8();
            if b == b'\x00' {
                break;
            }
            let mut label = vec![0u8; b as usize];
            data.read_exact(&mut label)?;
            name.push_str(str::from_utf8(&label)?);
            name.push('.');
        }
        name.pop();
        let val = data.get_u16();
        let qtype = RfcType::from_u16(val)?;
        let val = data.get_u16();
        let qclass = RfcClass::from_u16(val)?;
        Ok(Self {
            name,
            qtype,
            qclass,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for label in self.name.split('.') {
            res.push(label.len() as u8);
            res.extend(label.as_bytes());
        }
        res.push(0);
        res.extend(self.qtype.as_u16().to_be_bytes());
        res.extend(self.qclass.as_u16().to_be_bytes());
        res
    }
}
