use std::io::{Read, Write};
use std::net::TcpListener;

pub fn start_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;

    println!("Nebula network server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connection from {:?}", stream.peer_addr());

                let mut buffer = [0u8; 1024];

                let bytes_read = stream.read(&mut buffer)?;

                if bytes_read == 0 {
                    continue;
                }

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);

                match message.trim() {
                    "HELLO" => {
                        println!("Received HELLO");

                        stream.write_all(b"HELLO_ACK")?;
                    }

                    _ => {
                        println!("Unknown message: {}", message.trim());

                        stream.write_all(b"UNKNOWN_MESSAGE")?;
                    }
                }
            }

            Err(error) => {
                eprintln!("Connection failed: {}", error);
            }
        }
    }

    Ok(())
}