mod components;
mod frame_types;
mod errors;

use cookie_factory::GenError;
pub(crate) use errors::SerializationError;
pub use frame_types::*;

pub type GenCursor<'a> = (&'a mut [u8], usize);
pub type GenResult<'a> = Result<GenCursor<'a>, GenError>;