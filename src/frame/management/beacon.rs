use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct Beacon {
    pub header: ManagementHeader,
    pub timestamp: u64,
    pub beacon_interval: u16,
    pub capability_info: CapabilityInfo,
    pub station_info: StationInfo,
}
