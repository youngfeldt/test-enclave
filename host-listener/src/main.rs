use std::io::{self, Read};
use std::net::Shutdown;
use vsock::{VsockListener, VsockStream};
use serde_json::Value;

fn handle_client(mut stream: VsockStream) -> io::Result<()> {
    let mut buffer = Vec::new();
    
    // Read the incoming data from the enclave
    stream.read_to_end(&mut buffer)?;

    // Try to deserialize the data into a JSON object for printing
    if let Ok(response) = serde_json::from_slice::<Value>(&buffer) {
        println!("Received attestation document: {:?}", response);
    } else {
        println!("Failed to parse received data.");
    }

    // Close the connection
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let vsock_port = 5000; // Must match the port used by the enclave

    // Set up a VSOCK listener on the specified port
    let listener = VsockListener::bind(vsock_port)?;

    println!("Listening for VSOCK connections on port {}", vsock_port);

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established.");
                // Handle the client connection
                if let Err(e) = handle_client(stream) {
                    eprintln!("Failed to handle client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
