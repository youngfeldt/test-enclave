FROM amazonlinux:2

# Install necessary tools and dependencies
RUN yum update -y && \
    amazon-linux-extras enable aws-nitro-enclaves-cli && \ 
    yum install -y \
    aws-nitro-enclaves-cli \
    musl-devel \
    && yum clean all

# Copy the Rust binary into the Docker image
COPY ./target/release/nitro-attestation /app/nitro-attestation

# Keep the enclave alive and open a shell for manual execution
CMD ["/bin/bash"]
