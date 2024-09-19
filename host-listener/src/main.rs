use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use vsock::VsockListener;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024]; // Adjust buffer size if needed
    match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                let received_data = String::from_utf8_lossy(&buffer[..size]);
                println!("Received: {}", received_data);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    // Use VsockListener to bind to the VSOCK CID and port where the enclave sends data
    let listener = VsockListener::bind(vsock::Cid::Local, 5005)?; // Use the correct VSOCK port

    println!("Host is listening on VSOCK port 5005...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
