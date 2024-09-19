.ONESHELL:

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean:  ## Clean up rust target and remove keys
	cd nitro-attestation && cargo && clean cd ..
	cd host-listener && cargo clean && cd ..
	rm -fv *.pem
	rm -fv *.eif

build_nitro_attestation: ## Builds rust program that will reside in enclave and get attestation doc
	cd nitro-attestation && cargo build --release
	cd ..

build_listener:
	@cd host-listener && cargo build --release

image: build_nitro_attestation  ## Builds docker image
	cd nitro-attestation 
	docker build -t nitro-attestation-console .


eif: image keys nitro-attestation-console-signed.eif  ## Builds EIF and signs it
	@echo eif

keys: my_priv_key.pem csr.pem certificate.pem  ## Builds keys to sign image with
	@echo "making keys"

enclave: eif listener ## Runs Nitro Enclave
# Make idempotent
	nitro-cli run-enclave --cpu-count 2 --memory 2500 --eif-path ./nitro-attestation-console-signed.eif
	nitro-cli describe-enclaves

##################
certificate.pem:
	@# Sign it
	@echo "Creating signed certificate"
	@openssl x509 -req -days 20  -in csr.pem -out certificate.pem -sha384 -signkey my_priv_key.pem

my_priv_key.pem:
	@# Create private key (with elyptic curve)
	@echo "Creating private key"
	@openssl ecparam -name secp384r1 -genkey -out my_priv_key.pem

csr.pem::
	@# Gen CSR
	@echo "creating CSR"
	@openssl req -new -key my_priv_key.pem -sha384 -nodes -subj "/CN=AWS/C=US/ST=WA/L=Seattle/O=Amazon/OU=AWS" -out csr.pem

nitro-attestation-console-signed.eif:
	nitro-cli build-enclave --docker-uri nitro-attestation-console:latest --output-file nitro-attestation-console-signed.eif \
	--private-key my_priv_key.pem --signing-certificate certificate.pem

install.sh:
	@cat << EOF > install.sh
	sudo yum update -y
	sudo amazon-linux-extras install -y aws-nitro-enclaves-cli rust1 docker
	sudo yum install -y aws-nitro-enclaves-cli-devel
	sudo yum install -y gcc make
	# sudo amazon-linux-extras install -y docker
    sudo yum install -y wget openssl-devel readline-devel
	sudo systemctl start  docker
	sudo systemctl enable docker
	sudo usermod -a -G docker $USER
	sudo yum groupinstall -y "Development Tools"
	sudo yum install -y kernel-headers kernel-devel
	echo;echo;
	echo "update /etc/nitro_enclaves/allocator.yaml"
	echo
	EOF

socat: 
	wget http://www.dest-unreach.org/socat/download/socat-1.7.4.4.tar.gz

	tar xzf socat-1.7.4.4.tar.gz
	cd socat-1.7.4.4
	./configure
	make
	sudo make install 
	socat -V
	which socat



#.PHONY
listener:  ## SOCAT process to lisen and print output
	echo "STARTING LISTENER"
	socat VSOCK-LISTEN:5000,fork EXEC:"cat" &