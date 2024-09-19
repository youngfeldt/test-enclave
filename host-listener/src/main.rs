use vsock::VsockListener;
use std::io::Read;

fn main() {
    let port: u32 = 5005;  // Use the same port as in the enclave
    let listener = VsockListener::bind(port).expect("Failed to bind to VSOCK port");
    println!("Listening on VSOCK port {}", port);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).expect("Failed to read from stream");
        println!("Received attestation document: {:?}", buffer);
    }
}
