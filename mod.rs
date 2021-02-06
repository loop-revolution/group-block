use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{
			card::{error_card, CardComponent, CardHeader, Icon},
			stack::{StackComponent, StackDirection},
			text::TextComponent,
			DisplayComponent, WrappedComponent,
		},
		CreationObject, DisplayMeta, DisplayObject, PageMeta,
	},
	dsl::prelude::*,
	models::{Block, MinNewBlock, Property},
	schema::{blocks, properties},
	BlockError, Error,
};
use serde::{Deserialize, Serialize};

use crate::delegation::display::delegate_embed_display;
pub struct GroupBlock {}

pub const BLOCK_NAME: &'static str = "group";

fn group_properties(
	block_id: i64,
	conn: &PgConnection,
) -> Result<(Option<Block>, Vec<Block>), Error> {
	let block_properties: Vec<Property> = properties::dsl::properties
		.filter(properties::dsl::parent_id.eq(block_id))
		.load::<Property>(conn)?;

	let mut name: Option<Block> = None;
	let mut items: Vec<Block> = vec![];

	for property in block_properties {
		if property.property_name == "name" {
			name = blocks::dsl::blocks
				.filter(blocks::id.eq(property.value_id))
				.limit(1)
				.get_result(conn)
				.optional()?;
		} else if property.property_name == "item" {
			let block: Option<Block> = blocks::dsl::blocks
				.filter(blocks::id.eq(property.value_id))
				.limit(1)
				.get_result(conn)
				.optional()?;
			if let Some(block) = block {
				items.push(block);
			}
		}
	}

	Ok((name, items))
}

impl BlockType for GroupBlock {
	fn name() -> String {
		BLOCK_NAME.to_string()
	}

	fn info() -> TypeInfo {
		TypeInfo {
			name: Self::name(),
			icon: Icon::Folder,
			desc: "Group blocks organize blocks into folders.".to_string(),
		}
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
		let conn = &context.pool.get()?;
		let (name, items) = group_properties(block.id, conn)?;

		let name = name.and_then(|block| block.block_data);

		let name = match name {
			Some(string) => string,
			None => "Untitled Group".into(),
		};

		let items: Vec<WrappedComponent> = items
			.into_iter()
			.map(|block| WrappedComponent::from(delegate_embed_display(&block, context)))
			.collect();

		let content = StackComponent {
			direction: StackDirection::Fit,
			items,
		};

		Ok(DisplayObject::new(Box::new(content))
			.meta(DisplayMeta::new().page(PageMeta::new().header(&name))))
	}

	fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
		embed_display(block, context).unwrap_or_else(|e| Box::new(error_card(&e.to_string())))
	}

	fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
		Ok(CreationObject {
			header_component: Box::new(TextComponent::new("New Group Block")),
			main_component: Box::new(TextComponent::new("Coming soon")),
			input_template: "".to_string(),
		})
	}

	fn create(_input: String, _context: &Context, _user_id: i32) -> Result<Block, Error> {
		Err(Error::GenericError)
	}

	fn method_delegate(
		_context: &Context,
		name: String,
		_block_id: i64,
		_args: String,
	) -> Result<Block, Error> {
		match name.as_str() {
			_ => Err(BlockError::MethodExist(name, Self::name()).into()),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct CreationArgs {
	name: String,
	content: String,
}

fn embed_display(block: &Block, context: &Context) -> Result<Box<dyn DisplayComponent>, Error> {
	let conn = &context.pool.get()?;
	let (name, items) = group_properties(block.id, conn)?;

	let name = name.and_then(|block| block.block_data);

	let name = match name {
		Some(string) => string,
		None => "Untitled Group".into(),
	};

	let items: Vec<WrappedComponent> = items
		.into_iter()
		.map(|block| WrappedComponent::from(delegate_embed_display(&block, context)))
		.collect();

	let content = StackComponent {
		direction: StackDirection::Fit,
		items,
	};

	Ok(Box::new(CardComponent {
		color: None,
		content: Box::new(content),
		header: CardHeader::new(&name).id(block.id).icon(Icon::Folder),
	}))
}

impl GroupBlock {
	pub fn new(conn: &PgConnection, owner_id: i32) -> Result<Block, Error> {
		MinNewBlock {
			block_type: "group",
			owner_id,
		}
		.insert(conn)
	}
}
