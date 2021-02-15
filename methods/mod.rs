use super::BLOCK_NAME;
use block_tools::{blocks::Context, models::Block, BlockError, Error};
pub mod create;

pub fn method_delegate(
	_context: &Context,
	name: String,
	_block_id: i64,
	_args: String,
) -> Result<Block, Error> {
	Err(BlockError::MethodExist(name, BLOCK_NAME.to_string()).into())
}
