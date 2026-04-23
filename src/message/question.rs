use std::io::{Cursor, Read};

use anyhow::Result;
use bytes::Buf;

use crate::message::rfc::{RfcClass, RfcType};

pub struct Question {
    pub labels: Vec<String>,
    pub qtype: RfcType,
    pub qclass: RfcClass,
}

impl Question {
    pub fn parse(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut labels = Vec::new();
        while data.has_remaining() {
            let byte = data.get_u8();
            if byte == b'\x00' {
                break;
            } else if byte & 0b1100_0000 == 0b1100_0000 {
                let byte_1 = byte & 0b0011_1111;
                let byte_2 = data.get_u8();
                let offset = u16::from_be_bytes([byte_1, byte_2]);
                Self::parse_compressed(offset as u64, data, &mut labels)?;
                break;
            } else {
                let mut label = vec![0u8; byte as usize];
                data.read_exact(&mut label)?;
                labels.push(String::from_utf8(label)?);
            }
        }
        let val = data.get_u16();
        let qtype = RfcType::from_u16(val)?;
        let val = data.get_u16();
        let qclass = RfcClass::from_u16(val)?;
        Ok(Self {
            labels,
            qtype,
            qclass,
        })
    }

    fn parse_compressed(
        offset: u64,
        data: &mut Cursor<&[u8]>,
        labels: &mut Vec<String>,
    ) -> Result<()> {
        let pin_position = data.position();
        data.set_position(offset);
        while data.position() < pin_position - 1 {
            let byte = data.get_u8();
            if byte == b'\x00' {
                break;
            } else {
                let mut label = vec![0u8; byte as usize];
                data.read_exact(&mut label)?;
                labels.push(String::from_utf8(label)?);
            }
        }
        data.set_position(pin_position);
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut res = Vec::new();
        for label in self.labels.iter() {
            res.push(label.len() as u8);
            res.extend(label.as_bytes());
        }
        res.push(0);
        res.extend(self.qtype.as_u16().to_be_bytes());
        res.extend(self.qclass.as_u16().to_be_bytes());
        res
    }
}
