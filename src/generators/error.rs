use cookie_factory::GenError;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive, Debug, thiserror::Error)]
#[repr(u32)]
pub enum SerializationError {
    #[error("Management Frame Info Element length exceeds length limit (255 bytes)!")]
    MgmtElementTooLong = 0,
    #[error("Serialization of frame type 4 unsupported!")]
    FCtrlUnknownType,
    #[error("Invalid frame control subtype for frame type!")]
    FCtrlInvalidSubtype,
    #[error("Can't serialize reserved frame subtype!")]
    FCtrlReservedSubtype,
    #[error("Can't serialize unhandled frame subtype!")]
    FCtrlUnhandledSubtype,
}

impl From<SerializationError> for GenError {
    fn from(err: SerializationError) -> Self {
        GenError::CustomError(err.into())
    }
}
