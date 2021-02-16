use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{
			card::{error_card, Icon},
			DisplayComponent,
		},
		CreationObject, DisplayObject,
	},
	dsl::prelude::*,
	models::{Block, MinNewBlock},
	Error,
};
use display::{create::create_display, embed::embed_display, page::page_display};
use group_props::Properties;
use methods::method_delegate;
mod display;
mod group_props;
mod methods;
pub use methods::root::create_root;

pub const BLOCK_NAME: &str = "group";

pub struct GroupBlock {}
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

	fn block_name(block: &Block, context: &Context) -> Result<String, Error> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;
		let name = Properties::build(block.id, user_id, conn)?
			.name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Group Block".into());

		Ok(name)
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
		page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
		embed_display(block, context).unwrap_or_else(|e| box error_card(&e.to_string()))
	}

	fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error> {
		create_display(context, user_id)
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error> {
		methods::create::create(input, context, user_id)
	}

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, Error> {
		method_delegate(context, name, block_id, args)
	}
}

impl GroupBlock {
	pub fn insert_new(conn: &PgConnection, owner_id: i32) -> Result<Block, Error> {
		MinNewBlock {
			block_type: "group",
			owner_id,
		}
		.insert(conn)
	}
}
