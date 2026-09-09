use std::io::{Read, Write};
use std::net::TcpListener;

pub fn start_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;

    println!("TCP server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Incoming connection from {:?}", stream.peer_addr());

                let mut buffer = [0u8; 1024];

                let bytes_read = stream.read(&mut buffer)?;

                if bytes_read > 0 {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);

                    println!("Received: {}", message);

                    stream.write_all(b"HELLO_FROM_NEBULA")?;
                }
            }

            Err(error) => {
                eprintln!("Connection failed: {}", error);
            }
        }
    }

    Ok(())
}