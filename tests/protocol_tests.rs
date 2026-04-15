use crate::wire::packet::Packet;
use crate::wire::frame::Frame;
use crate::wire::codec::Codec;

#[test]
fn test_frame_encode_decode() {
    let packet = Packet::new(1, 0x02, b"test data".to_vec());
    let frame = Frame::new(packet.clone());

    let encoded = Codec::encode(&frame);
    let decoded = Codec::decode(&encoded).unwrap();

    assert_eq!(decoded.packet.version, packet.version);
    assert_eq!(decoded.packet.packet_type, packet.packet_type);
    assert_eq!(decoded.packet.payload, packet.payload);
}
