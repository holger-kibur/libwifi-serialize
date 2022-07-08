use cookie_factory::{do_gen, gen_le_u64, gen_le_u16, gen_call};

use crate::{generators::{GenCursor, GenResult}, frame::Beacon};
use crate::generators::components::{gen_mgmt_header, gen_station_info};

pub fn gen_beacon<'a>(cursor: GenCursor<'a>, frame: &Beacon) -> GenResult<'a> {
    do_gen!(cursor,
        gen_call!(gen_mgmt_header, &frame.header)
        >> gen_le_u64!(frame.timestamp)
        >> gen_le_u16!(frame.beacon_interval)
        >> gen_le_u16!(frame.capability_info)
        >> gen_call!(gen_station_info, &frame.station_info))
}