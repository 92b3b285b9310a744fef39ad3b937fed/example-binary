use nbtscanner::{run, Config};
use std::net::Ipv4Addr;

fn main() {
    run(vec![Ipv4Addr::new(127, 0, 0, 1)], Config::new(true));
}
