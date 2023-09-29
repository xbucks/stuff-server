use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn print(bytes: &[u8]) {
    match std::str::from_utf8(bytes) {
        Ok(string) => { println!("PRINT {}", string); }
        Err(_) => { println!("PRINT ERROR") }
    }
}

pub fn tcp_client(mut stream: TcpStream) {
    let mut receive_buffer = [0; 4098];

    println!("----->>> {:?}", stream.local_addr());
    loop {
        match stream.read(&mut receive_buffer) {
            Ok(received_size) => {
                if received_size == 0 {
                    return
                }

                let received_data = &receive_buffer[0..received_size];
                match stream.write(received_data) {
                    Ok(send_size) => {
                        if send_size != received_size {
                            println!("** STREAM RESEND ERROR **");
                            return
                        }

                        println!("** STREAM PING PONG **");
                        print(received_data);
                    }
                    Err(_) => {
                        println!("** STREAM STOPPED (WRITE) **");
                        return
                    }
                }
            }
            Err(_) => {
                println!("** STREAM STOPPED (READ) **");
                return
            }
        }
    }
}