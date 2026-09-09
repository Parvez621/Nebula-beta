use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect(address: &str) -> std::io::Result<()> {
    println!("Connecting to {}", address);

    let mut stream = TcpStream::connect(address)?;

    stream.write_all(b"HELLO_FROM_NEBULA")?;

    let mut buffer = [0u8; 1024];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read > 0 {
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("Server response: {}", response);
    }

    Ok(())
}