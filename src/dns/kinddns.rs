use super::{DnsHeader,DnsQuestion,DnsRecord,QueryType};
use std::io::Result;
use super::super::bytepacketbuffer::BytePacketBuffer;
#[derive(Clone,Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }
    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;
        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(),QueryType::UNKOWN(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }

        for _ in 0..result.header.authority_rrs {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }

        for _ in 0..result.header.additional_rrs {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }
        Ok(result)
    }

    pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authority_rrs = self.authorities.len() as u16;
        self.header.additional_rrs = self.resources.len() as u16;
        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }
        for answer in &self.answers {
            answer.write(buffer)?;
        }

        for authoritie in &self.authorities {
            authoritie.write(buffer)?;
        }

        for resource in &self.resources {
            resource.write(buffer)?;
        }
        Ok(())
    }
}
