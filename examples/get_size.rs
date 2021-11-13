use libnbd_rs;

fn main() {
    let mut h = libnbd_rs::NbdHandle::create();
    h.connect_command(&[
        "nbdkit",
        "-s",
        "--exit-with-parent",
        "null",
        &format!("size={}", 1048576),
    ]);

    println!("get_size {}", h.get_size());
    h.close();
}
