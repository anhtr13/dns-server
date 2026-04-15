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

pub enum QuestionType {
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

impl QuestionType {
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

pub enum QuestionClass {
    In,
    Cs,
    Ch,
    Hs,
}

impl QuestionClass {
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
    pub name: String,
    pub qtype: QuestionType,
    pub class: QuestionClass,
}

impl Question {
    pub fn new(name: String, qtype: u16, class: u16) -> Result<Self> {
        let qtype = QuestionType::from_u16(qtype)?;
        let class = QuestionClass::from_u16(class)?;
        Ok(Self { name, qtype, class })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for label in self.name.split('.') {
            res.push(label.len() as u8);
            res.extend(label.as_bytes());
        }
        res.push(0);
        res.extend(self.qtype.to_u16().to_be_bytes());
        res.extend(self.class.to_u16().to_be_bytes());
        res
    }
}

pub struct Message {
    header: Header,
    question: Question,
}

impl Message {
    pub fn new(header: Header, question: Question) -> Self {
        Self { header, question }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.serialize());
        res.extend(self.question.serialize());
        res
    }
}
