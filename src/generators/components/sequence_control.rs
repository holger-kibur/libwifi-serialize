use cookie_factory::{do_gen, gen_le_u8};

use crate::{generators::{GenCursor, GenResult}, frame::components::SequenceControl};

pub fn gen_sequence_ctrl<'a>(cursor: GenCursor<'a>, seq_ctrl: &SequenceControl) -> GenResult<'a> {
    // 4 bit fragment number + upper 4 bits of sequence number
    let seq_ctrl_byte_1 = (seq_ctrl.fragment_number << 4) | ((seq_ctrl.sequence_number >> 8) as u8 & 0x0F);
    // Lower 8 bits of sequence number
    let seq_ctrl_byte_2 = seq_ctrl.sequence_number as u8;
    do_gen!(cursor, gen_le_u8!(seq_ctrl_byte_1) >> gen_le_u8!(seq_ctrl_byte_2))
}