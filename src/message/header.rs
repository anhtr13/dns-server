use std::io::Cursor;

use bytes::Buf;

#[derive(Debug, Clone)]
pub struct Header {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            id: 1234,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

impl Header {
    pub fn parse(reader: &mut Cursor<&[u8]>) -> Self {
        let id = reader.get_u16();
        let byte = reader.get_u8();
        let qr = (byte & 0b1000_0000) >> 7;
        let opcode = (byte & 0b0111_1000) >> 3;
        let aa = (byte & 0b0000_0100) >> 2;
        let tc = (byte & 0b0000_0010) >> 1;
        let rd = byte & 1;
        let byte = reader.get_u8();
        let ra = (byte & 0b1000_0000) >> 7;
        let z = (byte & 0b0111_0000) >> 4;
        let rcode = byte & 0b0000_1111;
        let qdcount = reader.get_u16();
        let ancount = reader.get_u16();
        let nscount = reader.get_u16();
        let arcount = reader.get_u16();
        Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend(self.id.to_be_bytes());
        let mut byte = 0u8;
        byte |= (self.qr & 1) << 7;
        byte |= (self.opcode & 0b0000_1111) << 3;
        byte |= (self.aa & 1) << 2;
        byte |= (self.tc & 1) << 1;
        byte |= self.rd & 1;
        bytes.push(byte);
        let mut byte = 0u8;
        byte |= (self.ra & 1) << 7;
        byte |= (self.z & 0b0000_0111) << 4;
        byte |= self.rcode & 0b0000_1111;
        bytes.push(byte);
        bytes.extend(self.qdcount.to_be_bytes());
        bytes.extend(self.ancount.to_be_bytes());
        bytes.extend(self.nscount.to_be_bytes());
        bytes.extend(self.arcount.to_be_bytes());
        bytes
    }
}
