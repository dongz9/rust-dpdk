extern crate libc;

use libc::{c_int, size_t, c_void};

#[repr(C)]
pub struct RteCtrlMbuf {
    data: *mut c_void,
    data_len: u32,
}

#[repr(C)]
pub struct RtePktMbuf {

}

pub type MARKER   = [*mut c_void; 0usize];
pub type MARKER8  = [u8; 0usize];
pub type MARKER64 = [u64; 0usize];

#[repr(C)]
pub struct RtePktMbuf {
    pub cacheline0: MARKER,
    pub buf_addr: *mut c_void,
    
    
    pub next: *mut RteMbuf,
    pub data: *mut c_void,
    pub data_len: u16,

    pub num_segs: u8,
    pub in_port: u8,
    pub pkt_len: u32,
}

#[repr(C)]
pub struct RtePktMbufPollPrivate {
    mbuf_data_room_size: u16,
}

#[repr(C)]

#[repr(C)]
pub struct UdpHdr {
    pub src_port: u16,
    pub dst_port: u16,
    pub dgram_len: u16,
    pub dgrarm_cksum: u16,
}

#[repr(C)]
pub struct TcpHdr {
    pub src_port: u16,
    pub dst_port: u16,
    pub sent_seq: u32,
    pub recv_ack: u32,
    pub data_off: u8,
    pub tcp_flags: u8,
    pub rx_win: u16,
    pub tcp_urp: u16,
}
