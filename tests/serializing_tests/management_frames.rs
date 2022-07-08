use libwifi::frame::components::{ManagementHeader, FrameControl, SequenceControl, StationInfo, SupportedRate, ManagementInfoId};
use libwifi::frame::{Frame, Beacon};
use libwifi::serialize_frame;
use pretty_hex::pretty_hex;
use crate::*;


#[test]
fn serialize_beacon() {
    let beacon = Frame::Beacon(Beacon {
        header: ManagementHeader {
            frame_control: FrameControl {
                protocol_version: 0,
                frame_type: libwifi::FrameType::Management,
                frame_subtype: libwifi::FrameSubType::Beacon,
                flags: 0xAA,
            },
            duration: [0xF0, 0x0D],
            address_1: TEST_MAC_1,
            address_2: TEST_MAC_2,
            address_3: TEST_MAC_3,
            sequence_control: SequenceControl {
                fragment_number: 0xF1,
                sequence_number: 0xF234,
            }
        },
        timestamp: 0x0123456789ABCDEF,
        beacon_interval: 0xBEAC,
        capability_info: 0x2BAD,
        station_info: StationInfo {
            ssid: Some("My face when internet protocol".to_owned()),
            supported_rates: vec![SupportedRate(0x0C), SupportedRate(0x12), SupportedRate(0x0C), SupportedRate(0x12)],
            data: vec![(ManagementInfoId::TIM, vec![0, 3, 1, 0]), (ManagementInfoId::DsParameterSet, vec![9, 17])],
        }
    });
    let mut buffer = [0_u8; 2304];
    let bytes_written = serialize_frame(buffer.as_mut_slice(), &beacon).expect("Couldn't serialize beacon frame!");
    let frame_raw = &buffer[..bytes_written];
    println!("{}", pretty_hex(&frame_raw));
}