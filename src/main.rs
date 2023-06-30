use std::io::{stdin, stdout, Read, Write};

fn main() {
    let (stdin, mut stdout) = (stdin(), stdout());
    let mut buf = [0_u8; 4096];
    loop {
        match stdin.lock().read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                stdout
                    .write_all(&buf[..n].escape_ascii().collect::<Vec<u8>>())
                    .ok();
                stdout.flush().ok();
            }
        }
    }
}
