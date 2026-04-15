use emp::wire::codec::Codec;
use emp::wire::frame::Frame;

fn main() {
    println!("EMP Packet Inspector");

    // Example raw packet (you can replace with real input)
    let raw = vec![
        0x45, 0x4D, // Magic "EM"
        0x01,       // Version
        0x02,       // Type
        0x00, 0x00, 0x00, 0x04, // Length
        0x74, 0x65, 0x73, 0x74, // "test"
    ];

    match Codec::decode(&raw) {
        Ok(frame) => {
            println!("Decoded Frame:");
            println!("Version: {}", frame.packet.version);
            println!("Type: {}", frame.packet.packet_type);
            println!("Payload: {:?}", String::from_utf8_lossy(&frame.packet.payload));
        }
        Err(e) => {
            println!("Failed to decode: {}", e);
        }
    }
}
