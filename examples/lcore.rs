extern crate dpdk;

fn main() {
    dpdk::eal_init(std::env::args());

    let num_lcores = dpdk::lcore_count();
    println!("{}", num_lcores);

    for lcore_id in 0..num_lcores {
        if dpdk::lcore_is_enabled(lcore_id) {
            println!("{} is enabled", lcore_id);
        }
    }
}
