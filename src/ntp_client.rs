use std::net::UdpSocket;

#[derive(Debug)]
pub struct NtpDataPacket {
    li_vn_mode: u8,
    strat: u8, 
    poll: u8,
    prec: u8,
    root_delay: u32,
    root_dispersion: u32,
    ref_id: u32,
    ref_time_seconds: u32,
    ref_time_fraction: u32,
    origin_ts_seconds: u32,
    origin_ts_fraction: u32,
    recv_ts_seconds: u32,
    recv_ts_fraction: u32,
    tx_ts_seconds: u32,
    tx_ts_fraction: u32,
}

pub fn write_data_packet(socket: &UdpSocket) {
    let mut buf = [0; 48];
    buf[0] = 0x1b;
    socket.send(&buf).expect("Issue with sending to server");
}

pub fn read_data_packet(socket: &UdpSocket) -> [u8; 48] {
    let mut buf: [u8; 48] = [0; 48];
    if let Ok(r) = socket.recv(&mut buf) { 
        println!("Received: {} bytes", r);
    } else { 
        println!("Error receiving from server!");
    }

    return buf;
}

impl NtpDataPacket {

    pub fn new() -> Self {
        Self { li_vn_mode: 0, strat: 0, poll: 0, prec: 0, root_delay: 0, root_dispersion: 0, ref_id: 0, ref_time_seconds: 0, ref_time_fraction: 0, origin_ts_seconds: 0, origin_ts_fraction: 0, recv_ts_seconds: 0, recv_ts_fraction: 0, tx_ts_seconds: 0, tx_ts_fraction: 0 }
    }

    pub fn parse_response(&mut self, resp: [u8; 48]) {
        self.li_vn_mode = resp[0];
        self.strat = resp[1];
        self.poll = resp[2];
        self.prec = resp[3];
        self.root_delay |= (resp[4] as u32) << 24;
        self.root_delay |= (resp[5] as u32) << 16;
        self.root_delay |= (resp[6] as u32) << 8;
        self.root_delay |= resp[7] as u32;
        self.root_dispersion |= (resp[8] as u32) << 24;
        self.root_dispersion |= (resp[9] as u32) << 16;
        self.root_dispersion |= (resp[10] as u32) << 8;
        self.root_dispersion |= resp[11] as u32;
        self.ref_id |= (resp[12] as u32) << 24;
        self.ref_id |= (resp[13] as u32) << 16;
        self.ref_id |= (resp[14] as u32) << 8;
        self.ref_id |= resp[15] as u32;
        self.ref_time_seconds |= (resp[16] as u32) << 24;
        self.ref_time_seconds |= (resp[17] as u32) << 16;
        self.ref_time_seconds |= (resp[18] as u32) << 8;
        self.ref_time_seconds |= resp[19] as u32;
        self.ref_time_fraction |= (resp[20] as u32) << 24;
        self.ref_time_fraction |= (resp[21] as u32) << 16;
        self.ref_time_fraction |= (resp[22] as u32) << 8;
        self.ref_time_fraction |= resp[23] as u32;
        self.origin_ts_seconds |= (resp[24] as u32) << 24;
        self.origin_ts_seconds |= (resp[25] as u32) << 16;
        self.origin_ts_seconds |= (resp[26] as u32) << 8;
        self.origin_ts_seconds |= resp[27] as u32;
        self.origin_ts_fraction |= (resp[28] as u32) << 24;
        self.origin_ts_fraction |= (resp[29] as u32) << 16;
        self.origin_ts_fraction |= (resp[30] as u32) << 8;
        self.origin_ts_fraction |= resp[31] as u32;
        self.recv_ts_seconds |= (resp[32] as u32) << 24;
        self.recv_ts_seconds |= (resp[33] as u32) << 16;
        self.recv_ts_seconds |= (resp[34] as u32) << 8;
        self.recv_ts_seconds |= resp[35] as u32;
        self.recv_ts_fraction |= (resp[36] as u32) << 24;
        self.recv_ts_fraction |= (resp[37] as u32) << 16;
        self.recv_ts_fraction |= (resp[38] as u32) << 8;
        self.recv_ts_fraction |= resp[39] as u32;
        self.tx_ts_seconds |= (resp[40] as u32) << 24;
        self.tx_ts_seconds |= (resp[41] as u32) << 16;
        self.tx_ts_seconds |= (resp[42] as u32) << 8;
        self.tx_ts_seconds |= resp[43] as u32;
        self.tx_ts_fraction |= (resp[44] as u32) << 24;
        self.tx_ts_fraction |= (resp[45] as u32) << 16;
        self.tx_ts_fraction |= (resp[46] as u32) << 8;
        self.tx_ts_fraction |= resp[47] as u32;
    }

}