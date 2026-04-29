use std::io::{Cursor, Read};

use anyhow::Result;
use bytes::Buf;

use crate::message::rfc::{RfcClass, RfcType};

#[derive(Clone, Debug)]
pub struct Answer {
    pub labels: Vec<String>,
    pub atype: RfcType,
    pub aclass: RfcClass,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl Answer {
    pub fn default() -> Self {
        Self {
            labels: Vec::new(),
            atype: RfcType::A,
            aclass: RfcClass::In,
            ttl: 60,
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn parse(reader: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut labels = Vec::new();
        while reader.has_remaining() {
            let byte = reader.get_u8();
            if byte == b'\x00' {
                break;
            }
            let mut label = vec![0u8; byte as usize];
            reader.read_exact(&mut label)?;
            labels.push(String::from_utf8(label)?);
        }
        let atype = RfcType::from_u16(reader.get_u16())?;
        let aclass = RfcClass::from_u16(reader.get_u16())?;
        let ttl = reader.get_u32();
        let length = reader.get_u16();
        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;
        Ok(Self {
            labels,
            atype,
            aclass,
            ttl,
            length,
            data,
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.labels.into_iter() {
            bytes.push(label.len() as u8);
            bytes.extend(label.into_bytes());
        }
        bytes.push(0);
        bytes.extend(self.atype.as_u16().to_be_bytes());
        bytes.extend(self.aclass.as_u16().to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.length.to_be_bytes());
        bytes.extend(self.data);
        bytes
    }
}
