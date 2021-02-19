use crate::{
	blocks::{data_block::edit_data_component, group_block::group_props::Properties},
	delegation::display::delegate_embed_display,
};
use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::{
		component::{
			menu::MenuComponent,
			stack::{StackComponent, StackDirection},
			text::TextComponent,
			DisplayComponent, WrappedComponent,
		},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::Block,
	Error,
};

pub fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
	let conn = &context.conn()?;
	let user_id = optional_validate_token(optional_token(context))?;

	let Properties {
		name,
		description,
		items,
	} = Properties::build(block.id, user_id, conn)?;

	let name = name
		.and_then(|block| block.block_data)
		.unwrap_or_else(|| "Untitled Group".into());
	let desc = description.clone().and_then(|block| block.block_data);
	let items: Vec<WrappedComponent> = items
		.into_iter()
		.map(|block| WrappedComponent::from(delegate_embed_display(&block, context)))
		.collect();

	let stack: Box<dyn DisplayComponent> = if items.is_empty() {
		box TextComponent::new("No items in group")
	} else {
		box StackComponent {
			direction: StackDirection::Fit,
			items,
		}
	};
	let mut content = StackComponent::new(StackDirection::Vertical);

	if let Some(desc) = desc {
		let block = description.unwrap();
		content.push(
			box edit_data_component(block.id.to_string())
				.label("Description")
				.initial_value(&desc),
		)
	}
	content.push(stack);

	let mut page = PageMeta::new().header(&name);

	if let Some(user_id) = user_id {
		page.menu = Some(MenuComponent::load_from_block(block, user_id));
	}

	Ok(DisplayObject::new(box content).meta(DisplayMeta::default().page(page)))
}
