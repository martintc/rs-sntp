use std::net::UdpSocket;

use rs_sntp::ntp_client::{write_data_packet, read_data_packet, NtpDataPacket};

const HOST: &str = "0.pool.ntp.org";
const PORT: &str = "123";

fn main() {

    let mut packet = NtpDataPacket::new();
    
    let connection_string = HOST.to_owned() + ":" + PORT;

    let mut socket: UdpSocket = UdpSocket::bind("0.0.0.0:8000")
        .expect("Could not bind to socket");

    socket.connect(connection_string).expect("Could not connect to server");

    write_data_packet(&socket);

    println!("Data sent");

    let response = read_data_packet(&socket);

    println!("Data reseived");

    packet.parse_response(response);

    println!("{:?}", packet);



}
