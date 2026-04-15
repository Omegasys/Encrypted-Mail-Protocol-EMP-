use emp::crypto::asymmetric::KeyPair;
use emp::crypto::signatures::SignatureKeyPair;

fn main() {
    let enc_keys = KeyPair::generate();
    let sig_keys = SignatureKeyPair::generate();

    println!("=== EMP Key Generation ===");

    println!("Encryption Public Key: {:02x?}", enc_keys.public.to_bytes());
    println!("Encryption Private Key: {:02x?}", enc_keys.private.to_bytes());

    println!("Signature Public Key: {:02x?}", sig_keys.verifying.to_bytes());
    println!("Signature Private Key: {:02x?}", sig_keys.signing.to_bytes());
}
