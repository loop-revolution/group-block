use super::GroupBlock;
use block_tools::{blocks::Context, models::Block, BlockError, Error};
pub mod add;
pub mod create;
pub mod root;
pub mod visibility_update;
use block_tools::blocks::BlockType;

pub fn method_delegate(
	context: &Context,
	name: String,
	block_id: i64,
	args: String,
) -> Result<Block, Error> {
	match name.as_str() {
		"add" => add::add_method(context, block_id, args),
		_ => Err(BlockError::MethodExist(name, GroupBlock::name()).into()),
	}
}
