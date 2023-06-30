use std::io::{stdin, stdout, Read, Write};

fn main() {
    let (stdin, mut stdout) = (stdin(), stdout());
    let mut inbuf = [0_u8; 1024];
    let mut outbuf = [0_u8; 4096];
    loop {
        match stdin.lock().read(&mut inbuf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                stdout
                    .write_all({
                        let write_bytes = inbuf[..n].escape_ascii().fold(0, |idx, byte| {
                            outbuf[idx] = byte;
                            idx + 1
                        });

                        &outbuf[..write_bytes]
                    })
                    .ok();
                stdout.flush().ok();
            }
        }
    }
}
