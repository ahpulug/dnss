use std::net::UdpSocket;
fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("0.0.0.0:53")?;
    let mut buf = [0; 300];
    let (amt,src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];
    println!("{:?}", buf);
    Ok(())
}