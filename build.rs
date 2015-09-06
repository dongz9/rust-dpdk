// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let dpdk_dir = Path::new(&manifest_dir)
        .join("3ps/dpdk/x86_64-native-linuxapp-gcc/lib");

    println!("cargo:rustc-link-search={}", dpdk_dir.display());
    println!("cargo:rustc-link-lib=static=rte_eal");
    println!("cargo:rustc-link-lib=static=ethdev");
}
