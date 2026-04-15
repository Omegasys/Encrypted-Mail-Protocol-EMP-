use crate::onion::layering::OnionLayer;

#[test]
fn test_onion_encrypt_decrypt() {
    let data = b"secret message".to_vec();

    let keys = vec![
        b"key1".to_vec(),
        b"key2".to_vec(),
        b"key3".to_vec(),
    ];

    let encrypted = OnionLayer::encrypt_layers(data.clone(), keys.clone());

    let mut decrypted = encrypted;

    for key in keys {
        decrypted = OnionLayer::decrypt_layer(decrypted, key).unwrap();
    }

    assert_eq!(decrypted, data);
}
