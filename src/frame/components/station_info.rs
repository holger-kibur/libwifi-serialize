use num_enum::{TryFromPrimitive, IntoPrimitive};

use crate::error::Error;

#[derive(Clone, Debug, Default)]
/// StationInfo is used to parse and store variable length fields that are often sent
/// with management frames.
///
/// Each field has an `id`, the length of the bytes for this field, and then payload of the field.
/// Since there's a large number of possible fields and many propriatary vendor-specific usages
/// of these fields, this generic solution is used to capture all of them.
///
/// It is also important to note that most of these fields won't be sent most of the time. \
/// All fields that are already handled by this library get their own field in the StationInfo
/// struct.
///
/// Since we cannot handle all all those elements, the bytes of all unhandled elements will
/// be saved in the `data` field under the respectiv element id.
pub struct StationInfo {
    /// The transmission rates that are supported by the station.
    /// Empty if no rates were transmitted.
    pub supported_rates: Vec<SupportedRate>,
    /// If the sender included a SSID, it will be in here.
    pub ssid: Option<String>,
    /// This map contains all fields that aren't explicitly parsed by us.
    /// The format is Vec<(FieldId, PayloadBytes)>.
    ///
    /// Please consider to create a PR, if you write a parser for a new field :).
    pub data: Vec<(ManagementInfoId, Vec<u8>)>,
}

#[derive(Clone, Debug, Default)]
pub struct SupportedRate(pub u8);

impl TryFrom<SupportedRate> for f32 {
    type Error = Error;

    /// Supported rate in Mbps.
    fn try_from(value: SupportedRate) -> Result<Self, Self::Error> {
        match value.0 {
            0x82 => Ok(1.0),
            0x84 => Ok(2.0),
            0x8b => Ok(5.5),
            0x0c => Ok(6.0),
            0x12 => Ok(9.0),
            0x96 => Ok(11.0),
            0x18 => Ok(12.0),
            0x24 => Ok(18.0),
            0x2c => Ok(22.0),
            0x30 => Ok(24.0),
            0x42 => Ok(33.0),
            0x48 => Ok(36.0),
            0x60 => Ok(48.0),
            0x6c => Ok(54.0),
            _ => Err(Error::Failure("Unknown supported rate value!".to_string(), vec![value.0])),
        }
    }
}

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
/// Management Frame Information Element IDs for tagged data in management frame
/// headers.
/// 
/// Reference: 802.11 Wireless Networks The Definitive Guide, Chapter 4 Table 7
/// 
/// There are some peculiarities with referenced table, namely elements with IDs
/// that fall within reserved ranges. I'm not sure what the deal with that is. I
/// assume that the element IDs are correct, and the reserved ranges are
/// interrupted, which is why this enum doesn't exactly match the table.
pub enum ManagementInfoId {
    SSID = 0,
    SupportedRates,
    FhParameterSet,
    DsParameterSet,
    CfParameterSet,
    TIM,
    IbssParamterSet,
    Country,
    HoppingPatternParams,
    HoppingPatternTable,
    Request,
    // 11-15 reserved
    ChallengeText = 16,
    // 17-31 reserved
    PowerConstraint = 32,
    PowerCapability,
    TpcRequest,
    TpcReport,
    SupportedChannels,
    ChannelSwitch,
    MeasureRequest,
    MeasureReport,
    Quiet,
    IbssDfs,
    ErpInfo,
    // 43-47 reserved
    RobustSecurityNetwork = 48,
    // 49 reserved
    ExtSupportedRates = 50,
    // 51-220 reserved
    WPA = 221,
    // 222-255 reserved
}