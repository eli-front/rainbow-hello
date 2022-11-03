use std::io::{self, Write};
use std::{thread, time};

fn main() {
    // print rainbow
    let colors = [
        "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m",
    ];
    let mut i = 0;
    loop {
        print!("{}{}", colors[i], "hello world");
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        print!("\x1b[2K\r");
        i = (i + 1) % colors.len();
    }
}
