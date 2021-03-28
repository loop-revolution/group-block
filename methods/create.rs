use block_tools::{
	blocks::Context,
	models::{Block, MinNewBlock},
	BlockError, Error,
};
use serde::{Deserialize, Serialize};

use crate::blocks::{data_block, group_block::BLOCK_NAME};

pub fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
	let input = serde_json::from_str::<CreationArgs>(&input);
	let input: CreationArgs = input.map_err(|_| BlockError::InputParse)?;

	create_with_args(input, context, user_id)
}

pub fn create_with_args(
	input: CreationArgs,
	context: &Context,
	user_id: i32,
) -> Result<Block, Error> {
	let conn = &context.conn()?;

	let group_block = MinNewBlock {
		block_type: BLOCK_NAME,
		owner_id: user_id,
	}
	.insert(conn)?;

	if let Some(name) = input.name {
		let name_block = MinNewBlock {
			block_type: data_block::BLOCK_NAME,
			owner_id: user_id,
		}
		.into()
		.data(&name)
		.insert(conn)?;

		group_block
			.make_property("name", name_block.id)
			.insert(conn)?;
	}

	if let Some(desc) = input.desc {
		let desc_block = MinNewBlock {
			block_type: data_block::BLOCK_NAME,
			owner_id: user_id,
		}
		.into()
		.data(&desc)
		.insert(conn)?;

		group_block
			.make_property("desc", desc_block.id)
			.insert(conn)?;
	}

	for item in input.items {
		let id: i64 = item.id.parse().unwrap();
		group_block.make_property("item", id).insert(conn)?;
	}

	Ok(group_block)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreationArgs {
	pub name: Option<String>,
	pub desc: Option<String>,
	pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
	pub id: String,
}
