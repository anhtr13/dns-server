pub struct MessageHeader {
    packet_id: u16,
    qr_indicator: u8,
    operation_code: u8,
    authoritative_answer: u8,
    truncation: u8,
    recursion_desired: u8,
    recursion_available: u8,
    reserved: u8,
    response_code: u8,
    question_count: u16,
    answer_record_count: u16,
    authority_record_count: u16,
    additional_record_count: u16,
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self {
            packet_id: 1234,
            qr_indicator: 1,
            operation_code: 0,
            authoritative_answer: 0,
            truncation: 0,
            recursion_desired: 0,
            recursion_available: 0,
            reserved: 0,
            response_code: 0,
            question_count: 0,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        }
    }
}

impl MessageHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(12);
        res.extend(self.packet_id.to_be_bytes());
        let mut second_bit = 0u8;
        second_bit |= (self.qr_indicator & 1) << 7;
        second_bit |= (self.operation_code & 0b0000_1111) << 3;
        second_bit |= (self.authoritative_answer & 1) << 2;
        second_bit |= (self.truncation & 1) << 1;
        second_bit |= self.recursion_desired & 1;
        res.push(second_bit);
        let mut third_bit = 0u8;
        third_bit |= (self.recursion_available & 1) << 7;
        third_bit |= (self.reserved & 0b0000_0111) << 4;
        third_bit |= self.response_code & 0b0000_1111;
        res.push(third_bit);
        res.extend(self.question_count.to_be_bytes());
        res.extend(self.answer_record_count.to_be_bytes());
        res.extend(self.authority_record_count.to_be_bytes());
        res.extend(self.additional_record_count.to_be_bytes());
        res
    }
}

pub struct Message {
    header: MessageHeader,
}

impl Message {
    pub fn new() -> Self {
        Self {
            header: MessageHeader::default(),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.serialize());
        res
    }
}
