use std::env;

use libnbd_rs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./get_size <uri>");
        return;
    }

    let uri = &args[1];
    let mut h = libnbd_rs::NbdHandle::create();
    h.connect_uri(uri);

    println!("get_size {}", h.get_size());
    h.close();
}
