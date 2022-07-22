use crate::*;
use libwifi::frame::components::{
    CapabilityInfo, FrameControl, ManagementHeader, ManagementInfoId, SequenceControl, StationInfo,
    SupportedRate,
};
use libwifi::frame::{Beacon, Frame};
use libwifi::serialize_frame;
use pretty_hex::pretty_hex;

#[test]
/// Test against a beacon frame captured with wireshark.
fn serialize_beacon() -> Result<(), libwifi::error::Error> {
    let ground_truth = hex::decode(
        "\
        80000000ffffffffffff14ebb6af7b67\
        14ebb6af7b6750db8871d8df04000000\
        640031140007626967204d4143010882\
        848b962430486c030109050400010000\
        230210002a010032040c121860301401\
        00000fac040100000fac040100000fac\
        020c000b0501000a0000460533000000\
        002d1aef1917ffff0000000000000000\
        000000000000000000000000004a0e14\
        000a002c01c8001400050019007f0805\
        00080000000040\
        ",
    )
    .expect("Couldn't decode ground truth hex!");

    type CI = CapabilityInfo;

    let beacon = Frame::Beacon(Beacon {
        header: ManagementHeader {
            frame_control: FrameControl {
                protocol_version: 0,
                frame_type: libwifi::FrameType::Management,
                frame_subtype: libwifi::FrameSubType::Beacon,
                flags: 0x00,
            },
            duration: [0x00, 0x00],
            address_1: MacAddress([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
            address_2: MacAddress([0x14, 0xeb, 0xb6, 0xaf, 0x7b, 0x67]),
            address_3: MacAddress([0x14, 0xeb, 0xb6, 0xaf, 0x7b, 0x67]),
            sequence_control: SequenceControl {
                fragment_number: 0,
                sequence_number: 3509,
            },
        },
        timestamp: 20935373192,
        beacon_interval: 100,
        capability_info: CI::ESS
            | CI::PRIVACY
            | CI::SHORT_PREAMBLE
            | CI::SHORT_TIME_SLOT
            | CI::MEASURE_RADIO,
        station_info: StationInfo {
            ssid: Some("big MAC".to_owned()),
            supported_rates: vec![
                1000.try_into()?,
                2000.try_into()?,
                5500.try_into()?,
                11000.try_into()?,
                18000.try_into()?,
                24000.try_into()?,
                36000.try_into()?,
                54000.try_into()?,
            ],
            data: vec![
                (ManagementInfoId::DsParameterSet, vec![0x09]),
                (ManagementInfoId::TIM, vec![0x00, 0x01, 0x00, 0x00]),
                (ManagementInfoId::TpcReport, vec![0x10, 0x00]),
                (ManagementInfoId::ErpInfo, vec![0x00]),
                (
                    ManagementInfoId::ExtSupportedRates,
                    vec![0x0c, 0x12, 0x18, 0x60],
                ),
                (
                    ManagementInfoId::RobustSecurityNetwork,
                    vec![
                        0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04,
                        0x01, 0x00, 0x00, 0x0f, 0xac, 0x02, 0x0c, 0x00,
                    ],
                ),
                (
                    ManagementInfoId::ObssLoadElement,
                    vec![0x01, 0x00, 0x0a, 0x00, 0x00],
                ),
                (
                    ManagementInfoId::RmEnabledCapability,
                    vec![0x33, 0x00, 0x00, 0x00, 0x00],
                ),
                (
                    ManagementInfoId::HtCapability,
                    vec![
                        0xef, 0x19, 0x17, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ],
                ),
            ],
        },
    });
    let mut buffer = [0_u8; 2304];
    let bytes_written =
        serialize_frame(buffer.as_mut_slice(), &beacon).expect("Couldn't serialize beacon frame!");
    let frame_raw = &buffer[..bytes_written];
    println!("{}", pretty_hex(&frame_raw));

    if !compare_byte_slice(frame_raw, &ground_truth[..bytes_written]) {
        panic!("Frame doesn't match ground truth!");
    }

    Ok(())
}
