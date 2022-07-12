use cookie_factory::{do_gen, gen_le_u8};

use crate::{
    frame::components::SequenceControl,
    generators::{GenCursor, GenResult},
};

pub fn gen_sequence_ctrl<'a>(cursor: GenCursor<'a>, seq_ctrl: &SequenceControl) -> GenResult<'a> {
    // Upper 8 bits of 12 bit sequence number
    let seq_ctrl_byte_1 = (seq_ctrl.sequence_number >> 4) as u8;
    // Lower 4 bits of sequence number + 4 bit fragment number
    let seq_ctrl_byte_2 = (seq_ctrl.sequence_number << 4) as u8 | (seq_ctrl.fragment_number & 0x0F);
    do_gen!(
        cursor,
        gen_le_u8!(seq_ctrl_byte_2) >> gen_le_u8!(seq_ctrl_byte_1)
    )
}
