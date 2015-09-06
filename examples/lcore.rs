extern crate dpdk;

fn main() {
    dpdk::eal_init(std::env::args());

    let num_lcores = dpdk::lcore_count();
    println!("{} lcores", num_lcores);
    println!("master lcore is {}", dpdk::get_master_lcore());

    for lcore_id in 0..num_lcores {
        if dpdk::lcore_is_enabled(lcore_id) {
            println!("{} is enabled, socket id is {}", lcore_id, dpdk::lcore_to_socket_id(lcore_id));
        }
    }
}
