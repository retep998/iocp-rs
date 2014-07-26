
extern crate iocp;
extern crate green;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, iocp::event_loop, main)
}

fn main() {
    println!("\
        ╔══════╗\n\
        ║Привет║\n\
        ╚══════╝\
    ");
}
