use block_tools::{blocks::Context, Error};

use crate::blocks::group_block::group_props::Properties;

pub fn visibility_update(context: &Context, block_id: i64, public: bool) -> Result<(), Error> {
	let conn = &context.conn()?;
	let Properties {
		name, description, ..
	} = Properties::get_dangerous(block_id, conn)?;

	if let Some(name) = name {
		name.update_public(public, conn)?;
	}

	if let Some(desc) = description {
		desc.update_public(public, conn)?;
	}

	Ok(())
}
