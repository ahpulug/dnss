
extern crate dnss;
use std::net::UdpSocket;
use dnss::dns::kinddns::DnsPacket;
use dnss::bytepacketbuffer::BytePacketBuffer;
use dnss::dns::*;
fn main() {
    let qname = "www.yahoo.com";
    let qtype = QueryType::A;
    let server = ("114.114.114.114",53);
    let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(DnsQuestion::new(qname.to_string(),qtype));
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();
    let res_packet = DnsPacket::from_buffer(&mut res_buffer).unwrap();
    println!("{:?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:?}", q);
    }
    for rec in res_packet.answers {
        println!("{:?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:?}", rec);
    }
    // let mut bytebuffer = BytePacketBuffer::new();    
    // let (amt,src) = socket.recv_from(&mut bytebuffer.buf)?;
    // let packet = DnsPacket::from_buffer(&mut bytebuffer)?;
    // println!("{:?}", packet.header);
    // for q in packet.questions {
    //     println!("{:?}", q);
    // }
    // for rec in packet.answers {
    //     println!("{:?}", rec);
    // }
    // for rec in packet.authorities {
    //     println!("{:?}", rec);
    // }
    // for rec in packet.resources {
    //     println!("{:?}", rec);
    // }
    // Ok(())
}