use crate::crypto::symmetric::SymmetricKey;
use crate::crypto::asymmetric::KeyPair;

#[test]
fn test_symmetric_encryption() {
    let key = SymmetricKey::generate();

    let plaintext = b"hello world";

    let (cipher, nonce) = key.encrypt(plaintext);

    let decrypted = key.decrypt(&cipher, &nonce).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_key_exchange() {
    let a = KeyPair::generate();
    let b = KeyPair::generate();

    let shared1 = a.shared_secret(&b.public);
    let shared2 = b.shared_secret(&a.public);

    assert_eq!(shared1, shared2);
}
