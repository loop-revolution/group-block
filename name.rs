use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	models::Block,
	Error,
};

pub fn block_name(block: &Block, context: &Context) -> Result<String, Error> {
	let conn = &context.conn()?;
	let user_id = optional_validate_token(optional_token(context))?;
	let name = super::group_props::Properties::build(block.id, user_id, conn)?
		.name
		.and_then(|block| block.block_data)
		.unwrap_or_else(|| "Group Block".into());

	Ok(name)
}
