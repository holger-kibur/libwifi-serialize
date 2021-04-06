use libwifi::{
    frame_types::{FrameSubType, FrameType},
    Frame,
};

#[test]
fn test_association_request() {
    let payload = [
        0, 0, 100, 0, 17, 1, 0, 15, 78, 117, 99, 108, 101, 97, 114, 95, 70, 97, 108, 108, 111, 117,
        116, 1, 8, 140, 18, 152, 36, 176, 72, 96, 108, 5, 4, 2, 3, 0, 0, 7, 52, 69, 85, 32, 36, 1,
        23, 40, 1, 23, 44, 1, 23, 48, 1, 23, 52, 1, 23, 56, 1, 23, 60, 1, 23, 64, 1, 23, 100, 1,
        30, 104, 1, 30, 108, 1, 30, 112, 1, 30, 116, 1, 30, 132, 1, 30, 136, 1, 30, 140, 1, 30, 0,
        32, 1, 0, 35, 2, 14, 0, 48, 20, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15, 172,
        2, 12, 0, 45, 26, 255, 9, 23, 255, 255, 255, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 61, 22, 40, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        127, 8, 0, 0, 0, 0, 0, 0, 0, 64, 221, 49, 0, 80, 242, 4, 16, 74, 0, 1, 16, 16, 68, 0, 1, 2,
        16, 71, 0, 16, 190, 15, 245, 213, 137, 177, 64, 140, 203, 243, 77, 29, 90, 130, 118, 247,
        16, 60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 0, 0, 28, 0, 0,
        221, 24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94, 0, 98, 50,
        47, 0,
    ];

    let (_, frame) = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(
        frame.control.frame_subtype,
        FrameSubType::AssociationRequest
    ));
}

#[test]
fn test_null_data() {
    let payload = [
        72, 17, 60, 0, 156, 128, 223, 131, 16, 180, 252, 25, 16, 16, 128, 171, 156, 128, 223, 131,
        16, 180, 128, 43,
    ];
}

#[test]
fn test_data() {
    let payload = [0, 0];
}

#[test]
fn test_RTS() {
    let payload = [
        180, 0, 158, 0, 116, 66, 127, 77, 29, 45, 20, 125, 218, 170, 84, 81,
    ];
}

#[test]
fn test_CTS() {
    // 2B FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let payload = [196, 0, 246, 14, 224, 62, 68, 8, 195, 239];
}

#[test]
fn test_ACK() {
    // 2B FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let payload = [212, 0, 0, 0, 104, 217, 60, 214, 195, 239];
}

#[test]
fn test_BlockAckRequest() {
    // 2B FrameControl + 2B Duration + 6B Address1 + 6B Address2 + 4B CRC
    let payload = [
        132, 0, 58, 1, 192, 238, 251, 75, 207, 58, 24, 29, 234, 198, 62, 190, 4, 0, 160, 15,
    ];
}
