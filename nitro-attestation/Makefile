.ONESHELL:

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean:  ## Clean up rust target and remove keys
	cargo clean
	rm -fv *.pem

build: ## Builds rust program that will reside in enclave and get attestation doc
	cargo build --release

image: build  ## Builds docker image
	docker build -t nitro_attestation-console .

eif: image keys  ## Builds EIF and signs it
	nitro-cli build-enclave --docker-uri nitro-attestation-console:latest --output-file nitro-attestation-console-signed.eif --private-key key_name.pem --signing-certificate certificate.pem

keys: my_priv_key.pem csr.pem certificate.pem  ## Builds keys to sign image with
	@echo "making keys"

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