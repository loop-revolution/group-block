use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{card::error_card, icon::Icon, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::{Block, MinNewBlock},
	Error,
};
mod display;
mod group_props;
mod methods;
mod name;
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
		name::block_name(block, context)
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
		display::page::page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent> {
		display::embed::embed_display(block, context)
			.unwrap_or_else(|e| box error_card(&e.to_string()))
	}

	fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error> {
		display::create::create_display(context, user_id)
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
		methods::method_delegate(context, name, block_id, args)
	}
}

impl GroupBlock {
	pub fn insert_new(
		conn: &block_tools::dsl::prelude::PgConnection,
		owner_id: i32,
	) -> Result<Block, Error> {
		MinNewBlock {
			block_type: "group",
			owner_id,
		}
		.insert(conn)
	}
}
