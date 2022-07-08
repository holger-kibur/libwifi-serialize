mod components;
mod frame_types;
mod errors;

use cookie_factory::GenError;
pub(crate) use errors::SerializationError;
pub use frame_types::*;

pub type GenCursor<'a> = (&'a mut [u8], usize);
// FIXME: Allow for more descriptive errors. propagate a custom error type
// instead of GenError, which only allocates custom codes. Investigate the
// gen_call! macro to see how error handling works there, maybe write a custom
// gen_call! if needed.
pub type GenResult<'a> = Result<GenCursor<'a>, GenError>;