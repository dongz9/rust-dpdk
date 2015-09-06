extern crate libc;

use libc::{c_int, c_uint, size_t, c_void};
use std::{env, ffi};

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

const RTE_MAX_LCORE: usize = 64usize;

#[repr(C)]
#[derive(PartialEq)]
enum RteLcoreRole {
    RoleRte,
    RoleOff,
}

#[repr(C)]
enum RteProcType {
    RteProcAuto = -1,
    RteProcPrimary = 0,
    RteProcSecondary,
    RteProcInvalid,
}

#[repr(packed)]
struct RteConfig {
    master_lcore: u32,
    lcore_count: u32,
    lcore_role: [RteLcoreRole; RTE_MAX_LCORE],
    process_type: RteProcType,
    flags: u32,
    mem_config: *const c_void,
}

#[repr(C)]
enum RteLcoreState {
    Wait,
    Running,
    Finished,
}

#[repr(C)]
pub struct LcoreConfig {
    detected: u32,
    thread_id: u64,
    pipe_master2slave: [i32; 2usize],
    pipe_slave2master: [i32; 2usize],
    f: *const c_void,
    arg: *const c_void,
    ret: i32,
    state: RteLcoreState,
    socket_id: u32,
    core_id: u32,
}

#[link(name = "rte_eal")]
#[link(name = "rte_mempool")]
#[link(name = "rte_ring")]
extern {
    fn rte_eal_get_configuration() -> *const RteConfig;

    fn rte_eal_init(argc: i32, argv: *const *const i8) -> i32;

    fn rte_set_log_level(level: u32) -> ();

    static lcore_config: [LcoreConfig; RTE_MAX_LCORE];
}

pub fn eal_init(args: env::Args) -> i32 {
    unsafe {
        let c_args: Vec<*const i8> = args
            .map(|arg| {
                ffi::CString::from_vec_unchecked(arg.into_bytes()).as_ptr()
            })
            .collect();

        rte_eal_init(c_args.len() as i32, c_args.as_ptr() as *const *const i8)
    }
}

fn eal_get_configuration() -> &'static RteConfig {
    unsafe {
        &(*rte_eal_get_configuration())
    }
}

pub fn lcore_count() -> u32 {
    eal_get_configuration().lcore_count
}

pub fn get_master_lcore() -> u32 {
    eal_get_configuration().master_lcore
}

pub fn lcore_is_enabled(lcore_id: u32) -> bool {
    let cfg = eal_get_configuration();

    if (lcore_id >= RTE_MAX_LCORE as u32) {
        false
    } else {
        cfg.lcore_role[lcore_id as usize] != RteLcoreRole::RoleOff
    }
}

pub fn lcore_to_socket_id(lcore_id: u32) -> u32 {
    unsafe {
        lcore_config[lcore_id as usize].socket_id
    }
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
    fn rte_eth_dev_count() -> u8;

    fn rte_eth_macaddr_get(port_id: u8, mac_addr: *mut EtherAddr) -> ();
}

pub fn eth_dev_count() -> u8 {
    unsafe {
        rte_eth_dev_count()
    }
}

