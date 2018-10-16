
extern crate native_tls;
extern crate dnss;
use std::net::{UdpSocket,TcpStream};
use std::io::{Read, Write};
use dnss::dns::kinddns::{UdpDns,TcpDns};
use dnss::bytepacketbuffer::BytePacketBuffer;
use dnss::dns::*;
use std::mem;
use native_tls::TlsConnector;
fn main() {
    let qname = "www.yahoo.com";
    let qtype = QueryType::A;
    let server = ("1.1.1.1",853);
    let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
    let mut bytebuffer = BytePacketBuffer::new();    
    let (amt,src) = socket.recv_from(&mut bytebuffer.buf).unwrap();
    let mut packet = UdpDns::from_buffer(&mut bytebuffer).unwrap();
    println!("{:?}", packet.header);
    for q in packet.questions.clone() {
        println!("{:?}", q);
    }
    for rec in packet.answers.clone() {
        println!("{:?}", rec);
    }
    for rec in packet.authorities.clone() {
        println!("{:?}", rec);
    }
    for rec in packet.resources.clone() {
        println!("{:?}", rec);
    }
    let mut packet = TcpDns::from_udp_dns(&mut packet).unwrap();
    println!("{}", packet.length);
    println!("{:?}", packet.header);
    for q in packet.questions.clone() {
        println!("{:?}", q);
    }
    for rec in packet.answers.clone() {
        println!("{:?}", rec);
    }
    for rec in packet.authorities.clone() {
        println!("{:?}", rec);
    }
    for rec in packet.resources.clone() {
        println!("{:?}", rec);
    }
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(server).unwrap();
    let mut stream = connector.connect("cloudflare-dns.com", stream).unwrap();
    let mut byte = BytePacketBuffer::new();
    packet.write(&mut byte).unwrap();
    stream.write_all(&mut byte.buf).unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("收到请求");
    println!("{}", String::from_utf8_lossy(&res));
    // let mut packet = UdpDns::new();
    // packet.header.id = 6666;
    // packet.header.questions = 1;
    // packet.header.recursion_desired = true;
    // packet.questions.push(DnsQuestion::new(qname.to_string(),qtype));
    // let mut req_buffer = BytePacketBuffer::new();
    // packet.write(&mut req_buffer).unwrap();
    // socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();
    // let mut res_buffer = BytePacketBuffer::new();
    // socket.recv_from(&mut res_buffer.buf).unwrap();
    // let res_packet = UdpDns::from_buffer(&mut res_buffer).unwrap();
    // println!("{:?}", res_packet.header);

    // for q in res_packet.questions {
    //     println!("{:?}", q);
    // }
    // for rec in res_packet.answers {
    //     println!("{:?}", rec);
    // }
    // for rec in res_packet.authorities {
    //     println!("{:?}", rec);
    // }
    // for rec in res_packet.resources {
    //     println!("{:?}", rec);
    // }

    // Ok(())
}