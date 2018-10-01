
extern crate dnss;
use std::net::UdpSocket;
use dnss::dns::kinddns::DnsPacket;
use dnss::bytepacketbuffer::BytePacketBuffer;
fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("0.0.0.0:53")?;
    let mut bytebuffer = BytePacketBuffer::new();    
    let (amt,src) = socket.recv_from(&mut bytebuffer.buf)?;
    let packet = DnsPacket::from_buffer(&mut bytebuffer)?;
    println!("{:?}", packet.header);
    for q in packet.questions {
        println!("{:?}", q);
    }
    for rec in packet.answers {
        println!("{:?}", rec);
    }
    for rec in packet.authorities {
        println!("{:?}", rec);
    }
    for rec in packet.resources {
        println!("{:?}", rec);
    }
    Ok(())
}