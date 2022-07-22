use num_enum::{IntoPrimitive, TryFromPrimitive};

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

impl TryFrom<SupportedRate> for u32 {
    type Error = Error;

    /// Supported rate in Kbps.
    fn try_from(value: SupportedRate) -> Result<Self, Self::Error> {
        match value.0 {
            0x82 => Ok(1000),
            0x84 => Ok(2000),
            0x8b => Ok(5500),
            0x0c => Ok(6000),
            0x12 => Ok(9000),
            0x96 => Ok(11000),
            0x18 => Ok(12000),
            0x24 => Ok(18000),
            0x2c => Ok(22000),
            0x30 => Ok(24000),
            0x42 => Ok(33000),
            0x48 => Ok(36000),
            0x60 => Ok(48000),
            0x6c => Ok(54000),
            _ => Err(Error::ParseFailure(
                "Unknown supported rate value!".to_string(),
                vec![value.0],
            )),
        }
    }
}

impl TryFrom<u32> for SupportedRate {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let rate = match value {
            1000 => Some(0x82),
            2000 => Some(0x84),
            5500 => Some(0x8b),
            6000 => Some(0x0c),
            9000 => Some(0x12),
            11000 => Some(0x96),
            12000 => Some(0x18),
            18000 => Some(0x24),
            22000 => Some(0x2c),
            24000 => Some(0x30),
            33000 => Some(0x42),
            36000 => Some(0x48),
            48000 => Some(0x60),
            54000 => Some(0x6c),
            _ => None,
        };
        rate.map_or_else(
            || {
                Err(Error::SerializeFailure(format!(
                    "Can't be an 802.11 supported rate: {} kbps!",
                    value
                )))
            },
            |x| Ok(SupportedRate(x)),
        )
    }
}

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
/// Management Frame Information Element IDs for tagged data in management frame
/// headers.
///
/// References:
///  - 802.11 Wireless Networks The Definitive Guide, Chapter 4 Table 7
///  - Wireshark
///
/// This table is an amalgamation of the table from the book, and info field
/// types from wireshark. Wireshark doesn't seem to maintain an internal wiki
/// page on the meanings of all these fields, and I don't care enough to crawl
/// through the source to find them, so this table is updated opportunistically.
///
/// Please update this table with new info element id types which you find in
/// the wild.
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
    ObssLoadElement,
    // 12-15 unknown
    ChallengeText = 16,
    // 17-31 unknown
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
    // 43-44 unknown
    HtCapability = 45,
    // 46-47 unknown
    RobustSecurityNetwork = 48,
    // 49 unknown
    ExtSupportedRates = 50,
    // 51-60 unknown
    HtInfo = 61,
    // 62-69 unknown
    RmEnabledCapability = 70,
    // 71-73 unknown
    BssOverlapParams = 74,
    // 75-220 unknown
    VendorSpecific = 221,
    // 222-254 unknown
    InfoIdExtension = 255,
}
