use std::env;

use libnbd_rs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./write <uri>");
        return;
    }

    let uri = &args[1];
    let mut h = libnbd_rs::NbdHandle::create();
    h.connect_uri(uri);

    let mut buf = [0u8; 512];
    for i in 0..512 {
        buf[i] = i as u8;
    }

    h.write(&mut buf, 512, 0, 0);

    let mut buf = [0u8; 512];
    h.read(&mut buf, 512, 0, 0);

    println!("{:?}", buf);

    h.close();
}
