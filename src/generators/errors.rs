use num_enum::{IntoPrimitive, TryFromPrimitive};

// TODO: Store error context information in SerializationError variants to give
// more helpful messages. This will require macros though.

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum SerializationError {
    MgmtElementTooLong = 0,
}

impl SerializationError {
    pub fn explain(self) -> &'static str {
        match self {
            Self::MgmtElementTooLong => "Management frame info element is too long! Limit is 255 bytes!",
        }
    }
}