
extern crate iocp;
extern crate green;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, iocp::event_loop, main)
}

fn main() {
    use std::io::stdio::{stdin, stdout};
    use std::str::UnicodeStrSlice;
    let (mut cin, mut cout) = (stdin(), stdout());
    (write!(cout, "In: ")).unwrap();
    cout.flush().unwrap();
    let msg = cin.read_line().unwrap();
    let msg = msg.as_slice().trim();
    let bar = String::from_char(msg.width(false), '═');
    (writeln!(cout, "╔{}╗", bar)).unwrap();
    (writeln!(cout, "║{}║", msg)).unwrap();
    (writeln!(cout, "╚{}╝", bar)).unwrap();
}
