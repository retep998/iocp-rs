
#![reexport_test_harness_main = "test_main"]

extern crate iocp;
extern crate green;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, iocp::event_loop,test_main)
}

#[test]
fn do_nothing() {
    ()
}

#[test]
fn print_something() {
    use std::io::stdio::stdout;
    let mut cout = stdout();
    cout.write_line("Привет, мир!").unwrap();
}
