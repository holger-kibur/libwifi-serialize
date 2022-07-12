use cookie_factory::{do_gen, gen_call, gen_cond, gen_le_u8, gen_many_ref, gen_slice, GenError};

use crate::frame::components::{ManagementInfoId, StationInfo, SupportedRate};
use crate::generators::{GenCursor, GenResult, SerializationError};

pub fn gen_station_info<'a>(cursor: GenCursor<'a>, station_info: &StationInfo) -> GenResult<'a> {
    do_gen!(
        cursor,
        // Generate ssid if present
        gen_cond!(station_info.ssid.is_some(), gen_call!(gen_ssid, station_info.ssid.as_ref().unwrap()))
        // Generate supported rates
        >> gen_supported_rates(&station_info.supported_rates)
        // Generate all other unparsed data fields
        >> gen_many_ref!(&station_info.data, gen_unparsed_field)
    )
}

fn gen_info_element_hdr(
    cursor: GenCursor<'_>,
    id: ManagementInfoId,
    data_len: usize,
) -> GenResult<'_> {
    let data_len: u8 = data_len
        .try_into()
        .map_err(|_| GenError::CustomError(SerializationError::MgmtElementTooLong.into()))?;
    do_gen!(
        cursor,
        // Generate element id
        gen_le_u8!(id.into())
        // Generate element byte len
        >> gen_le_u8!(data_len)
    )
}

fn gen_ssid<'a>(cursor: GenCursor<'a>, ssid: &String) -> GenResult<'a> {
    let ssid_utf8 = ssid.as_bytes();
    do_gen!(
        cursor,
        // Generate ssid element header
        gen_info_element_hdr(ManagementInfoId::SSID, ssid_utf8.len())
        // Generate ssid utf8 payload
        >> gen_slice!(ssid_utf8)
    )
}

fn gen_supported_rates<'a>(cursor: GenCursor<'a>, rates: &Vec<SupportedRate>) -> GenResult<'a> {
    do_gen!(
        cursor,
        // Generate supported rates element header
        gen_info_element_hdr(ManagementInfoId::SupportedRates, rates.len())
        // Generate supported rate payload bytes for each rate in the list
        >> gen_many_ref!(rates, |cursor, rate: &SupportedRate| gen_le_u8!(cursor, rate.0))
    )
}

fn gen_unparsed_field<'a>(
    cursor: GenCursor<'a>,
    field: &(ManagementInfoId, Vec<u8>),
) -> GenResult<'a> {
    do_gen!(
        cursor,
        // Generate unparsed element header
        gen_info_element_hdr(field.0, field.1.len())
        // Generate the payload
        >> gen_slice!(field.1.as_slice())
    )
}
