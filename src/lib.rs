extern crate libc;

use libc::{c_int, c_uint, size_t, c_void};

const RTE_MEMPOOL_CACHE_MAX_SIZE = 512usize;

// lib/librte_eal

const RTE_LOG_EMERG   = 1u32;
const RTE_LOG_ALERT   = 2u32;
const RTE_LOG_CRIT    = 3u32;
const RTE_LOG_ERR     = 4u32;
const RTE_LOG_WARNING = 5u32;
const RTE_LOG_NOTICE  = 6u32;
const RTE_LOG_INFO    = 7u32;
const RTE_LOG_DEBUG   = 8u32;

// #[repr(C)]
// pub struct RteCtrlMbuf {
//     data: *mut c_void,
//     data_len: u32,
// }

// #[repr(C)]
// pub struct RtePktMbuf {

// }

// pub type MARKER   = [*mut c_void; 0usize];
// pub type MARKER8  = [u8; 0usize];
// pub type MARKER64 = [u64; 0usize];

// #[repr(C)]
// pub struct RtePktMbuf {
//     pub cacheline0: MARKER,
//     pub buf_addr: *mut c_void,
    
    
//     pub next: *mut RteMbuf,
//     pub data: *mut c_void,
//     pub data_len: u16,

//     pub num_segs: u8,
//     pub in_port: u8,
//     pub pkt_len: u32,
// }

// #[repr(C)]
// pub struct RtePktMbufPollPrivate {
//     mbuf_data_room_size: u16,
// }

// // lib/librte_net

// #[repr(C)]
// pub struct Ipv4Hdr {
//     pub version_ihl: u8,
//     pub type_of_service: u8,
//     pub total_length: u16,
//     pub packet_id: u16,
//     pub fragment_offset: 16,
//     pub time_to_live: u8,
//     pub next_proto_id: u8,
//     pub hdr_checksum: u8,
//     pub src_addr: u32,
//     pub dst_addr: u32,
// }

// #[repr(C)]
// pub struct UdpHdr {
//     pub src_port: u16,
//     pub dst_port: u16,
//     pub dgram_len: u16,
//     pub dgrarm_cksum: u16,
// }

// #[repr(C)]
// pub struct TcpHdr {
//     pub src_port: u16,
//     pub dst_port: u16,
//     pub sent_seq: u32,
//     pub recv_ack: u32,
//     pub data_off: u8,
//     pub tcp_flags: u8,
//     pub rx_win: u16,
//     pub tcp_urp: u16,
// }

// lib/librte_ether
const ETHER_ADDR_LEN = 6usize;

#[repr(C)]
pub struct EtherAddr {
    pub addr_bytes: [u8; ETHER_ADDR_LEN],
}

// // lib/librte_mempool

// #[repr(C)]
// pub struct RteMempoolCache {
//     pub len: c_uint,
//     pub objs: [*mut c_void; RTE_MEMPOOL_CAHCE_MAX_SIZE * 3],
// }

#[link(name = "rte_eal")]
#[link(name = "rte_ether")]
extern {
    pub fn rte_set_log_level(level: u32) -> ();

    pub fn rte_lcore_count() -> u32;

    pub fn rte_get_master_lcore() -> u32;

    pub fn rte_eal_init(argc: i32, argv: *mut *mut i8) -> i32;

    pub fn rte_eth_dev_count() -> u8;

    pub fn rte_eth_macaddr_get(port_id: u8, mac_addr: *mut EtherAddr) -> ();
}
