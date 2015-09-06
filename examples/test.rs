extern crate dpdk;

use std::env;
use std::ffi;

fn main() {
    unsafe {
        let args: Vec<*const i8> = env::args()
            .map(|arg| {
                ffi::CString::from_vec_unchecked(arg.into_bytes()).as_ptr()
            })
            .collect();

        let ret = dpdk::rte_eal_init(args.len() as i32, args.as_ptr() as *const *const i8);
        println!("{}", ret);
    }
}
