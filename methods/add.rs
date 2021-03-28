use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	display_api::{
		component::search::{SearchComponent, SearchType},
		ActionObject, MethodObject,
	},
	models::Block,
	BlockError, Error,
};
use serde::{Deserialize, Serialize};

use crate::blocks::group_block::BLOCK_NAME;

pub fn add_action_object(block_id: i64) -> ActionObject {
	let method = MethodObject {
		block_type: BLOCK_NAME.into(),
		block_id: block_id.to_string(),
		method_name: "add".to_string(),
		arg_template: r#"{"id":$[ADD_BLOCK]$}"#.into(),
	};
	let search = SearchComponent::default()
		.action_text("Choose a Block to add")
		.r#type(SearchType::Block)
		.name("ADD_BLOCK")
		.then(ActionObject::method(method));
	ActionObject::search(search)
}

pub fn add_method(context: &Context, block_id: i64, args: String) -> Result<Block, Error> {
	let conn = &context.pool.get()?;
	let user_id = validate_token(&require_token(context)?)?;

	let access_err: Error =
		BlockError::TypeGenericError(format!("Cannot add blocks to {}", block_id)).into();

	let block = match Block::by_id(block_id, conn)? {
		Some(b) => b,
		None => return Err(access_err),
	};
	if !has_perm_level(user_id, &block, PermLevel::Edit) {
		return Err(access_err);
	}
	let invalid_err: Error = BlockError::InputParse.into();
	let input = match serde_json::from_str::<AddArgs>(&args) {
		Ok(input) => input,
		Err(_) => return Err(invalid_err),
	};
	let block_id: i64 = match input.id.parse() {
		Ok(id) => id,
		Err(_) => return Err(invalid_err),
	};
	block.make_property("item", block_id).insert(conn)?;
	Ok(block)
}

#[derive(Serialize, Deserialize, Debug)]
struct AddArgs {
	id: String,
}
