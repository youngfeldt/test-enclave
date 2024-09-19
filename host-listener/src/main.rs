use vsock::{VsockListener, VsockAddr};
use std::io::Read;

fn main() {
    let port: u32 = 5005;  // The port where the listener will bind
    let addr = VsockAddr::new(vsock::VMADDR_CID_ANY, port);  // Correct address format

    // Bind the listener to the port
    let listener = VsockListener::bind(&addr).expect("Failed to bind to VSOCK port");
    println!("Listening on VSOCK port {}", port);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).expect("Failed to read from stream");
        println!("Received attestation document: {:?}", buffer);
    }
}
