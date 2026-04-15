use anyhow::Result;

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

pub enum RfcType {
    A,
    Ns,
    Md,
    Mf,
    Cname,
    Soa,
    Mb,
    Mg,
    Mr,
    Null,
    Wks,
    Ptr,
    Hinfo,
    Minfo,
    Mx,
    Txt,
}

impl RfcType {
    pub fn from_u16(val: u16) -> Result<Self> {
        match val {
            1 => Ok(Self::A),
            2 => Ok(Self::Ns),
            3 => Ok(Self::Md),
            4 => Ok(Self::Mf),
            5 => Ok(Self::Cname),
            6 => Ok(Self::Soa),
            7 => Ok(Self::Mb),
            8 => Ok(Self::Mg),
            9 => Ok(Self::Mr),
            10 => Ok(Self::Null),
            11 => Ok(Self::Wks),
            12 => Ok(Self::Ptr),
            13 => Ok(Self::Hinfo),
            14 => Ok(Self::Minfo),
            15 => Ok(Self::Mx),
            16 => Ok(Self::Txt),
            _ => anyhow::bail!("unknow question type"),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match self {
            Self::A => 1,
            Self::Ns => 2,
            Self::Md => 3,
            Self::Mf => 4,
            Self::Cname => 5,
            Self::Soa => 6,
            Self::Mb => 7,
            Self::Mg => 8,
            Self::Mr => 9,
            Self::Null => 10,
            Self::Wks => 11,
            Self::Ptr => 12,
            Self::Hinfo => 13,
            Self::Minfo => 14,
            Self::Mx => 15,
            Self::Txt => 16,
        }
    }
}

pub enum RfcClass {
    In,
    Cs,
    Ch,
    Hs,
}

impl RfcClass {
    pub fn from_u16(val: u16) -> Result<Self> {
        match val {
            1 => Ok(Self::In),
            2 => Ok(Self::Cs),
            3 => Ok(Self::Ch),
            4 => Ok(Self::Hs),
            _ => anyhow::bail!("unknow question class"),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match self {
            Self::In => 1,
            Self::Cs => 2,
            Self::Ch => 3,
            Self::Hs => 4,
        }
    }
}

pub struct Question {
    name: String,
    qtype: RfcType,
    qclass: RfcClass,
}

impl Question {
    pub fn new(name: String, qtype: u16, qclass: u16) -> Result<Self> {
        let qtype = RfcType::from_u16(qtype)?;
        let qclass = RfcClass::from_u16(qclass)?;
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
        res.extend(self.qtype.to_u16().to_be_bytes());
        res.extend(self.qclass.to_u16().to_be_bytes());
        res
    }
}

pub struct Answer {
    name: String,
    atype: RfcType,
    aclass: RfcClass,
    ttl: u32,
    length: u16,
    data: Vec<u8>,
}

impl Answer {
    pub fn new(
        name: String,
        atype: u16,
        aclass: u16,
        ttl: u32,
        length: u16,
        data: Vec<u8>,
    ) -> Result<Self> {
        let atype = RfcType::from_u16(atype)?;
        let aclass = RfcClass::from_u16(aclass)?;
        Ok(Self {
            name,
            atype,
            aclass,
            ttl,
            length,
            data,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for label in self.name.split('.') {
            res.push(label.len() as u8);
            res.extend(label.as_bytes());
        }
        res.push(0);
        res.extend(self.atype.to_u16().to_be_bytes());
        res.extend(self.aclass.to_u16().to_be_bytes());
        res.extend(self.ttl.to_be_bytes());
        res.extend(self.length.to_be_bytes());
        res.extend(&self.data);
        res
    }
}

pub struct Message {
    header: Header,
    question: Question,
    answer: Answer,
}

impl Message {
    pub fn new(header: Header, question: Question, answer: Answer) -> Self {
        Self {
            header,
            question,
            answer,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.serialize());
        res.extend(self.question.serialize());
        res.extend(self.answer.serialize());
        res
    }
}
