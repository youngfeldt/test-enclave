use nsm_io::Request;
use serde_bytes::ByteBuf;
use std::io::{self, Write};
use vsock::VsockStream;

fn main() -> io::Result<()> {
    // Initialize NSM driver
    let nsm_fd = nsm_driver::nsm_init();

    // Create the attestation request
    let public_key = ByteBuf::from("my super secret key");
    let hello = ByteBuf::from("hello, world!");
    let request = Request::Attestation {
        public_key: Some(public_key),
        user_data: Some(hello),
        nonce: None,
    };

    // Get the attestation document from the NSM
    let response = nsm_driver::nsm_process_request(nsm_fd, request);
    println!("{:?}", response);

    // VSOCK connection parameters
    let host_cid: u32 = 3; // CID 3 is reserved for the host
    let vsock_port: u32 = 5000; // Define the port to connect to the host on

    // Create a VSOCK connection to the host
    let mut stream = VsockStream::connect(host_cid, vsock_port)?;

    // Serialize the response to a byte format (assuming the response can be serialized)
    let response_bytes = serde_json::to_vec(&response).expect("Failed to serialize response");

    // Send the attestation document to the host over VSOCK
    stream.write_all(&response_bytes)?;

    println!("Sent attestation document to host over VSOCK.");

    // Close the NSM session
    nsm_driver::nsm_exit(nsm_fd);

    Ok(())
}
