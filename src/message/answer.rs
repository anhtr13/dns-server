use crate::message::rfc::{RfcClass, RfcType};

pub struct Answer {
    pub name: String,
    pub atype: RfcType,
    pub aclass: RfcClass,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl Answer {
    pub fn default() -> Self {
        Self {
            name: String::new(),
            atype: RfcType::A,
            aclass: RfcClass::In,
            ttl: 60,
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for label in self.name.split('.') {
            res.push(label.len() as u8);
            res.extend(label.as_bytes());
        }
        res.push(0);
        res.extend(self.atype.as_u16().to_be_bytes());
        res.extend(self.aclass.as_u16().to_be_bytes());
        res.extend(self.ttl.to_be_bytes());
        res.extend(self.length.to_be_bytes());
        res.extend(&self.data);
        res
    }
}
