use block_tools::{
	blocks::Context,
	models::{Block, User},
	Error,
};

use super::create::{create_with_args, CreationArgs, Item};

pub fn create_root(context: &Context, user: User, first_block_id: i64) -> Result<Block, Error> {
	let conn = context.conn()?;
	let args = CreationArgs {
		name: Some("Dashboard".into()),
		desc: Some("".into()),
		items: vec![Item {
			id: first_block_id.to_string(),
		}],
	};
	let block = create_with_args(args, context, user.id)?;
	user.update_root(Some(block.id), &conn)?;
	Ok(block)
}
