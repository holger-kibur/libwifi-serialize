use nom::bytes::complete::take;
use nom::number::complete::u8 as get_u8;
use nom::sequence::tuple;
use nom::IResult;

use crate::frame::components::{ManagementInfoId, StationInfo, SupportedRate};

/// Parse variable length and variable field information.
/// The general structure of the data looks like this:
///
/// 1 byte: Element id
/// 1 byte: Element length (up to 255 bytes)
/// $element_length bytes: Element data
///
/// This format is only used in management frames.
///
/// There might be multiple elements with the same element id,
/// which is why StationInfo uses a Vec instead of BTreeMap as a data structure.
pub fn parse_station_info(mut input: &[u8]) -> IResult<&[u8], StationInfo> {
    let mut station_info = StationInfo::default();

    let mut element_id;
    let mut length;
    let mut data;
    loop {
        (input, (element_id, length)) = tuple((get_u8, get_u8))(input)?;
        //println!("Element id {}, Length: {}", element_id, length);
        (input, data) = take(length)(input)?;
        //println!("Extracted data: {:?}", data);

        if let Ok(element) = ManagementInfoId::try_from(element_id) {
            match element {
                ManagementInfoId::SSID => {
                    let mut ssid = String::from_utf8_lossy(data).to_string();
                    // Remove null chars. Some APs seem to enjoy sending those.
                    ssid = ssid.replace('\0', " ");
                    station_info.ssid = Some(ssid);
                }
                ManagementInfoId::SupportedRates => {
                    station_info.supported_rates =
                        data.iter().map(|rate| SupportedRate(*rate)).collect()
                }
                _ => {
                    station_info.data.push((element, data.to_vec()));
                }
            }
        } // TODO: Put some kind of opt-in warning here?

        if input.len() <= 4 {
            break;
        }
    }

    Ok((input, station_info))
}
