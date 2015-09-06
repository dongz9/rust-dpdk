extern crate libc;

use libc::{c_int, c_uint, size_t, c_void};

const RTE_MEMPOOL_CACHE_MAX_SIZE: usize = 512usize;

// lib/librte_eal

const RTE_LOG_EMERG: u32   = 1u32;
const RTE_LOG_ALERT: u32   = 2u32;
const RTE_LOG_CRIT: u32    = 3u32;
const RTE_LOG_ERR: u32     = 4u32;
const RTE_LOG_WARNING: u32 = 5u32;
const RTE_LOG_NOTICE: u32  = 6u32;
const RTE_LOG_INFO: u32    = 7u32;
const RTE_LOG_DEBUG: u32   = 8u32;

#[repr(C)]
pub struct RteConfig {
    pub master_lcore: u32,
    pub lcore_count: u32,
}

#[link(name = "rte_eal")]
#[link(name = "rte_mempool")]
#[link(name = "rte_ring")]
extern {
    pub fn rte_eal_get_configuration() -> *const RteConfig;

    pub fn rte_set_log_level(level: u32) -> ();

    pub fn rte_eal_init(argc: i32, argv: *const *const i8) -> i32;
}

pub fn eal_get_configuration() -> &'static RteConfig {
    unsafe {
        &(*rte_eal_get_configuration())
    }
}

pub fn lcore_count() -> u32 {
    let cfg = eal_get_configuration();

    cfg.lcore_count
}

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
const ETHER_ADDR_LEN: usize = 6usize;

#[repr(C)]
pub struct EtherAddr {
    pub addr_bytes: [u8; ETHER_ADDR_LEN],
}

#[link(name = "ethdev")]
extern {
    pub fn rte_eth_dev_count() -> u8;

    pub fn rte_eth_macaddr_get(port_id: u8, mac_addr: *mut EtherAddr) -> ();
}
