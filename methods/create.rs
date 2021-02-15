use block_tools::{
	blocks::Context,
	models::{Block, MinNewBlock},
	BlockError, Error, NoAccessSubject, UserError,
};
use serde::{Deserialize, Serialize};

use crate::blocks::{data_block, group_block::BLOCK_NAME};

pub fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
	let conn = &context.conn()?;

	let input = serde_json::from_str::<CreationArgs>(&input);

	let input: CreationArgs = input.map_err(|_| BlockError::InputParse)?;

	let group_block = MinNewBlock {
		block_type: BLOCK_NAME,
		owner_id: user_id,
	}
	.insert(conn)?;

	let name_block: Block;
	if input.name.perform == "CONNECT" {
		let id: i64 = input.name.data.parse().unwrap();
		name_block = match Block::by_id(id, conn)? {
			None => return Err(UserError::NoAccess(NoAccessSubject::ViewBlock(id)).into()),
			Some(block) => block,
		};
	} else {
		name_block = MinNewBlock {
			block_type: data_block::BLOCK_NAME,
			owner_id: user_id,
		}
		.into()
		.data(&input.name.data)
		.insert(conn)?;
	}

	let desc_block: Block;
	if input.desc.perform == "CONNECT" {
		let id: i64 = input.desc.data.parse().unwrap();
		desc_block = match Block::by_id(id, conn)? {
			None => return Err(UserError::NoAccess(NoAccessSubject::ViewBlock(id)).into()),
			Some(block) => block,
		};
	} else {
		desc_block = MinNewBlock {
			block_type: data_block::BLOCK_NAME,
			owner_id: user_id,
		}
		.into()
		.data(&input.desc.data)
		.insert(conn)?;
	}

	for item in input.items {
		let id: i64 = item.id.parse().unwrap();
		group_block.make_property("item", id).insert(conn)?;
	}

	group_block
		.make_property("name", name_block.id)
		.insert(conn)?;
	group_block
		.make_property("desc", desc_block.id)
		.insert(conn)?;

	Ok(group_block)
}

#[derive(Serialize, Deserialize, Debug)]
struct CreationArgs {
	name: DoData,
	desc: DoData,
	items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DoData {
	perform: String,
	data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
	id: String,
}
