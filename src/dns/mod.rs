use super::bytepacketbuffer::BytePacketBuffer;
use std::net::Ipv4Addr;
use std::io::Result;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[derive(Clone,Debug)]
pub struct DnsHeader {
    pub id: u16, //16 bit
    pub response: bool, //1 bit
    pub operation_code: u8, //4 bit
    pub authoritative_answer: bool, //1 bit
    pub truncated_message: bool, //1 bit
    pub recursion_desired: bool, //1 bit
    pub recursion_avilable: bool, //1 bit
    pub z:bool, //1 bit
    pub authed_data: bool, // 1 bit
    pub checking_disabled: bool, // 1 bit
    pub response_code: ResultCode, //4 bit
    pub questions: u16, //16 bit
    pub answers: u16, //16 bit
    pub authority_rrs: u16, //16 bit
    pub additional_rrs: u16, //16 bit
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,
            response: false,
            operation_code: 0,
            authoritative_answer: false,
            truncated_message:  false,
            recursion_desired: false,
            recursion_avilable: false,
            z: false,
            authed_data: false,
            checking_disabled: false,
            response_code: ResultCode::NOERROR,
            questions: 0,
            answers: 0,
            authority_rrs: 0,
            additional_rrs: 0,
        }
    }

    pub fn read(&mut self,buffer: &mut BytePacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;
        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xff) as u8;
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.operation_code = (a >> 3) & 0x0f;
        self.response = (a & (1 << 7)) > 0;
        self.response_code = ResultCode::from_num(b & 0x0f);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_avilable = (b & (1 << 7)) > 0;
        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authority_rrs = buffer.read_u16()?;
        self.additional_rrs = buffer.read_u16()?;
        Ok(())
    }
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Copy)]
pub enum QueryType {
    UNKOWN(u16),
    A,
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKOWN(x) => x,
            QueryType::A => 1,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType::UNKOWN(num),
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType
}

impl DnsQuestion {
    pub fn new(name: String,qtype: QueryType) -> DnsQuestion {
        DnsQuestion{
            name: name,
            qtype: qtype
        }
    }

    pub fn read(&mut self,buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?;
        Ok(())
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKOWN {
        domain: String,
        qtype: u16,
        ttl: u32,
        data_len: u16,
    },
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;
        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;
        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(((raw_addr >> 24) & 0xff) as u8,
                                         ((raw_addr >> 16) & 0xff) as u8,
                                         ((raw_addr >> 8) & 0xff) as u8,
                                         ((raw_addr >> 0) & 0xff) as u8,
                );
                Ok(DnsRecord::A {
                    domain: domain,
                    addr : addr,
                    ttl: ttl
                })
            },
            QueryType::UNKOWN(_) =>  {
                buffer.step(data_len as usize)?;
                Ok(DnsRecord::UNKOWN {
                    domain: domain,
                    qtype: qtype_num,
                    data_len: data_len,
                    ttl: ttl
                })
            }
        }
    }
}
