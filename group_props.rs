use block_tools::{
	auth::permissions::can_view,
	dsl::prelude::*,
	models::{Block, Property},
	schema::properties,
	Error,
};

pub struct Properties {
	pub name: Option<Block>,
	pub description: Option<Block>,
	pub items: Vec<Block>,
}

impl Default for Properties {
	fn default() -> Self {
		Properties {
			name: None,
			description: None,
			items: vec![],
		}
	}
}

impl Properties {
	pub fn build(
		block_id: i64,
		user_id: Option<i32>,
		conn: &PgConnection,
	) -> Result<Properties, Error> {
		let property_list: Vec<Property> = properties::dsl::properties
			.filter(properties::dsl::parent_id.eq(block_id))
			.load::<Property>(conn)?;

		let mut props = Properties::default();

		for property in property_list {
			match property.property_name.as_str() {
				"name" => {
					props.name = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
				}
				"desc" => {
					props.description = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
				}
				"item" => {
					let block = Block::by_id(property.value_id, conn)?
						.filter(|block| can_view(user_id, block));
					if let Some(block) = block {
						props.items.push(block);
					}
				}
				_ => {}
			}
		}

		Ok(props)
	}

	pub fn get_dangerous(block_id: i64, conn: &PgConnection) -> Result<Properties, Error> {
		let property_list: Vec<Property> = properties::dsl::properties
			.filter(properties::dsl::parent_id.eq(block_id))
			.load::<Property>(conn)?;

		let mut props = Properties::default();

		for property in property_list {
			match property.property_name.as_str() {
				"name" => {
					props.name = Block::by_id(property.value_id, conn)?;
				}
				"desc" => {
					props.description = Block::by_id(property.value_id, conn)?;
				}
				"item" => {
					let block = Block::by_id(property.value_id, conn)?;
					if let Some(block) = block {
						props.items.push(block);
					}
				}
				_ => {}
			}
		}

		Ok(props)
	}
}
