use emp::onion::layering::OnionLayer;

fn main() {
    let message = b"Hello EMP Onion Routing!".to_vec();

    let keys = vec![
        b"node1key".to_vec(),
        b"node2key".to_vec(),
        b"node3key".to_vec(),
    ];

    println!("Original: {:?}", String::from_utf8_lossy(&message));

    let encrypted = OnionLayer::encrypt_layers(message.clone(), keys.clone());

    println!("Encrypted: {:02x?}", encrypted);

    let mut decrypted = encrypted;

    for key in keys {
        decrypted = OnionLayer::decrypt_layer(decrypted, key).unwrap();
    }

    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
}
