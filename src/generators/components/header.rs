use cookie_factory::{GenError, do_gen, gen_call, gen_slice, gen_cond};

use crate::{generators::GenCursor, frame::components::{ManagementHeader, DataHeader}};

use super::{frame_control::gen_frame_control, sequence_control::gen_sequence_ctrl};

pub fn gen_management_header<'a>(cursor: GenCursor<'a>, mgmt_header: &ManagementHeader) -> Result<GenCursor<'a>, GenError> {
    do_gen!(cursor, 
        gen_frame_control(&mgmt_header.frame_control)
        >> gen_slice!(&mgmt_header.duration)
        >> gen_slice!(&mgmt_header.address_1.0)
        >> gen_slice!(&mgmt_header.address_2.0)
        >> gen_slice!(&mgmt_header.address_3.0)
        >> gen_sequence_ctrl(&mgmt_header.sequence_control))
}

pub fn gen_data_header<'a>(cursor: GenCursor<'a>, data_header: &DataHeader) -> Result<GenCursor<'a>, GenError> {
    do_gen!(cursor,
        gen_frame_control(&data_header.frame_control)
        >> gen_slice!(&data_header.duration)
        >> gen_slice!(&data_header.address_1.0)
        >> gen_slice!(&data_header.address_2.0)
        >> gen_slice!(&data_header.address_3.0)
        >> gen_sequence_ctrl(&data_header.sequence_control)
        >> gen_cond!(data_header.address_4.is_some(), gen_slice!(data_header.address_4.as_ref().unwrap().0))
        >> gen_cond!(data_header.qos.is_some(), gen_slice!(&data_header.qos.unwrap())))
}