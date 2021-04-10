use nom::number::complete::le_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::components::*;
use crate::parsers::{parse_management_header, parse_station_info};
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct AssociationRequest {
    pub header: ManagementHeader,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}

impl AssociationRequest {
    pub fn parse(input: &[u8]) -> IResult<&[u8], AssociationRequest> {
        let (input, (header, beacon_interval, capability_info, station_info)) =
            tuple((parse_management_header, le_u16, le_u16, parse_station_info))(input)?;

        Ok((
            input,
            AssociationRequest {
                header,
                beacon_interval,
                capability_info,
                station_info,
            },
        ))
    }
}

impl HasHeader for AssociationRequest {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}