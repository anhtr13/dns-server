use std::io::Cursor;

use bytes::Buf;

#[derive(Debug)]
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
    pub fn parse(data: &mut Cursor<&[u8]>) -> Self {
        let id = data.get_u16();
        let second_bit = data.get_u8();
        let qr = (second_bit & 0b1000_0000) >> 7;
        let opcode = (second_bit & 0b0111_1000) >> 3;
        let aa = (second_bit & 0b0000_0100) >> 2;
        let tc = (second_bit & 0b0000_0010) >> 1;
        let rd = second_bit & 1;
        let third_bit = data.get_u8();
        let ra = (third_bit & 0b1000_0000) >> 7;
        let z = (third_bit & 0b0111_0000) >> 4;
        let rcode = third_bit & 0b0000_1111;
        let qdcount = data.get_u16();
        let ancount = data.get_u16();
        let nscount = data.get_u16();
        let arcount = data.get_u16();
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

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(12);
        res.extend(self.id.to_be_bytes());
        let mut second_bit = 0u8;
        second_bit |= (self.qr & 1) << 7;
        second_bit |= (self.opcode & 0b0000_1111) << 3;
        second_bit |= (self.aa & 1) << 2;
        second_bit |= (self.tc & 1) << 1;
        second_bit |= self.rd & 1;
        res.push(second_bit);
        let mut third_bit = 0u8;
        third_bit |= (self.ra & 1) << 7;
        third_bit |= (self.z & 0b0000_0111) << 4;
        third_bit |= self.rcode & 0b0000_1111;
        res.push(third_bit);
        res.extend(self.qdcount.to_be_bytes());
        res.extend(self.ancount.to_be_bytes());
        res.extend(self.nscount.to_be_bytes());
        res.extend(self.arcount.to_be_bytes());
        res
    }
}
