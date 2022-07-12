use cookie_factory::{do_gen, gen_call, gen_le_u16, gen_le_u64};

use crate::generators::components::{gen_mgmt_header, gen_station_info};
use crate::{
    frame::Beacon,
    generators::{GenCursor, GenResult},
};

pub fn gen_beacon<'a>(cursor: GenCursor<'a>, frame: &Beacon) -> GenResult<'a> {
    do_gen!(
        cursor,
        gen_mgmt_header(&frame.header)
            >> gen_le_u64!(frame.timestamp)
            >> gen_le_u16!(frame.beacon_interval)
            >> gen_le_u16!(frame.capability_info)
            >> gen_station_info(&frame.station_info)
    )
}
