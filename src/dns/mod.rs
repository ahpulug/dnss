pub mod kinddns;
use super::bytepacketbuffer::BytePacketBuffer;
use std::net::{Ipv4Addr,Ipv6Addr};
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

    pub fn write(&self,buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_u16(self.id)?;

        buffer.write(((self.recursion_desired as u8)) |
                        ((self.truncated_message as u8) << 1) |
                        ((self.authoritative_answer as u8) << 2) |
                        (self.operation_code << 3) |
                        ((self.response as u8) << 7) as u8)?;

        buffer.write((self.response_code.clone() as u8) |
                        ((self.checking_disabled as u8) << 4) |
                        ((self.authed_data as u8) << 5) |
                        ((self.z as u8) << 6) |
                        ((self.recursion_avilable as u8) << 7) )?;

        buffer.write_u16(self.questions)?;
        buffer.write_u16(self.answers)?;
        buffer.write_u16(self.authority_rrs)?;
        buffer.write_u16(self.additional_rrs)?;

        Ok(())
    }
}

#[derive(PartialEq,Eq,Debug,Clone,Hash,Copy)]
pub enum QueryType {
    UNKOWN(u16),
    A, //id = 1
    NS, //id = 2
    CNAME, //id = 5
    MX, //id = 15
    AAAA, //id = 28
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKOWN(x) => x,
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::MX => 15,
            QueryType::AAAA => 28,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAAA,
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

    pub fn write(&self,buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_qname(&self.name)?;
        let typenum = self.qtype.to_num();
        buffer.write_u16(typenum)?;
        buffer.write_u16(1)?;
        Ok(())
    } 
}

#[derive(Debug,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
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
    },
    NS {
        domain: String,
        host: String,
        ttl: u32,
    },
    CNAME {
        domain: String,
        host: String,
        ttl: u32
    },
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    },
    AAAA {
        domain: String,
        addr: Ipv6Addr,
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
            QueryType::NS => {
                let mut ns = String::new();
                buffer.read_qname(&mut ns)?;
                Ok(DnsRecord::NS {
                    domain: domain,
                    host: ns,
                    ttl: ttl
                })
            },
            QueryType::CNAME => {
                let mut cname = String::new();
                buffer.read_qname(&mut cname)?;
                Ok(DnsRecord::NS {
                    domain: domain,
                    host: cname,
                    ttl: ttl
                })
            },
            QueryType::MX => {
                let priority = buffer.read_u16()?;
                let mut mx = String::new();
                buffer.read_qname(&mut mx)?;
                Ok(DnsRecord::MX {
                    domain: domain,
                    priority: priority,
                    host: mx,
                    ttl: ttl
                })
            },
            QueryType::AAAA => {
                let raw_addr1 = buffer.read_u32()?;
                let raw_addr2 = buffer.read_u32()?;
                let raw_addr3 = buffer.read_u32()?;
                let raw_addr4 = buffer.read_u32()?;
                let addr = Ipv6Addr::new(((raw_addr1 >> 16) & 0xFFFF) as u16,
                                     ((raw_addr1 >> 0) & 0xFFFF) as u16,
                                     ((raw_addr2 >> 16) & 0xFFFF) as u16,
                                     ((raw_addr2 >> 0) & 0xFFFF) as u16,
                                     ((raw_addr3 >> 16) & 0xFFFF) as u16,
                                     ((raw_addr3 >> 0) & 0xFFFF) as u16,
                                     ((raw_addr4 >> 16) & 0xFFFF) as u16,
                                     ((raw_addr4 >> 0) & 0xFFFF) as u16);

            Ok(DnsRecord::AAAA {
                domain: domain,
                addr: addr,
                ttl: ttl
            })
            }
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

    pub fn write(&self,buffer: &mut BytePacketBuffer) -> Result<()> {
        match *self {
            DnsRecord::A { ref domain, ref addr, ttl } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::A.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(4)?;

                let octets = addr.octets();
                buffer.write(octets[0])?;
                buffer.write(octets[1])?;
                buffer.write(octets[2])?;
                buffer.write(octets[3])?;
            },
            DnsRecord::NS { ref domain, ref host ,ttl } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::NS.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(QueryType::NS.to_num())?;
                buffer.write_qname(host)?;
                let size = buffer.pos() - pos - 2;
                buffer.set_u16(pos, size as u16)?;
            },
            DnsRecord::CNAME {ref domain, ref host, ttl } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::CNAME.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;
                buffer.write_qname(host)?;
                let size = buffer.pos() - pos -2;
                buffer.set_u16(pos, size as u16)?;
            },
            DnsRecord::MX { ref domain, priority, ref host, ttl } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::MX.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;

                let pos = buffer.pos();
                buffer.write_u16(0)?;
                buffer.write_u16(priority)?;
                buffer.write_qname(host)?;
                let size = buffer.pos() - pos - 2;
                buffer.set_u16(pos, size as u16)?;
            },
            DnsRecord::AAAA { ref domain, ref addr, ttl } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::AAAA.to_num())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(16)?;
                for octet in &addr.segments() {
                    buffer.write_u16(*octet)?;
                }
            },
            DnsRecord::UNKOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }
        Ok(())
    }
}

