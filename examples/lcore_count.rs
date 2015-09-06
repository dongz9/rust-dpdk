extern crate dpdk;

use std::env;
use std::ffi;
use dpdk::*;

fn main() {
    eal_init(env::args());

    println!("{}", lcore_count());
}
