use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect(address: &str) -> std::io::Result<()> {
    println!("Connecting to {}", address);

    let mut stream = TcpStream::connect(address)?;

    stream.write_all(b"HELLO")?;

    let mut buffer = [0u8; 1024];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        println!("Server closed the connection");
        return Ok(());
    }

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received: {}", response);

    if response.trim() == "HELLO_ACK" {
        println!("Nebula handshake successful");
    } else {
        println!("Unexpected response");
    }

    Ok(())
}