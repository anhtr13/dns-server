use std::io::{Cursor, Read};

use anyhow::{Context, Result};
use bytes::Buf;

use crate::message::rfc::{RfcClass, RfcType};

#[derive(Clone, Debug)]
pub struct Question {
    pub labels: Vec<String>,
    pub qtype: RfcType,
    pub qclass: RfcClass,
}

impl Question {
    pub fn parse(reader: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut labels = Vec::new();
        while reader.has_remaining() {
            let byte = reader.get_u8();
            if byte == b'\x00' {
                break;
            }
            if byte & 0b1100_0000 == 0b1100_0000 {
                let byte_1 = byte & 0b0011_1111;
                let byte_2 = reader.get_u8();
                let offset = u16::from_be_bytes([byte_1, byte_2]);
                Self::read_labels_from_pointer(reader, &mut labels, offset as u64)
                    .context("Failed to parse compressed labels")?;
                break;
            }
            let mut label = vec![0u8; byte as usize];
            reader.read_exact(&mut label)?;
            labels.push(String::from_utf8(label)?);
        }
        let qtype = RfcType::from_u16(reader.get_u16())?;
        let qclass = RfcClass::from_u16(reader.get_u16())?;
        Ok(Self {
            labels,
            qtype,
            qclass,
        })
    }

    fn read_labels_from_pointer(
        reader: &mut Cursor<&[u8]>,
        labels: &mut Vec<String>,
        offset: u64,
    ) -> Result<()> {
        let pinned_position = reader.position();
        reader.set_position(offset);
        while reader.position() < pinned_position - 1 {
            let byte = reader.get_u8();
            if byte == b'\x00' {
                break;
            }
            let mut label = vec![0u8; byte as usize];
            reader.read_exact(&mut label)?;
            labels.push(String::from_utf8(label)?);
        }
        reader.set_position(pinned_position);
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.labels.iter() {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }
        bytes.push(0);
        bytes.extend(self.qtype.as_u16().to_be_bytes());
        bytes.extend(self.qclass.as_u16().to_be_bytes());
        bytes
    }
}
