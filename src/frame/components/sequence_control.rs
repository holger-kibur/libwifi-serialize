#[derive(Clone, Debug)]
pub struct SequenceControl {
    /// The 4 bit fragment number from a sequence control field.
    pub fragment_number: u8,
    /// The 12 bit sequence number from a sequence control field.
    pub sequence_number: u16,
}

impl SequenceControl {
    pub fn create_fresh(seq_count: u16) -> Self {
        SequenceControl {
            fragment_number: 0,
            sequence_number: seq_count,
        }
    }
}
