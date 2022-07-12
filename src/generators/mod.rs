mod components;
mod error;
mod frame_types;

use cookie_factory::GenError;
pub(crate) use error::SerializationError;
pub use frame_types::*;

pub type GenCursor<'a> = (&'a mut [u8], usize);
pub type GenResult<'a> = Result<GenCursor<'a>, GenError>;
