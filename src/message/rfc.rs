use anyhow::Result;

#[derive(Clone, Copy)]
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
    pub fn as_u16(&self) -> u16 {
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

#[derive(Clone, Copy)]
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

    pub fn as_u16(&self) -> u16 {
        match self {
            Self::In => 1,
            Self::Cs => 2,
            Self::Ch => 3,
            Self::Hs => 4,
        }
    }
}
