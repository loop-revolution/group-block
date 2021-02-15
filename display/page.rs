use crate::{
	blocks::group_block::group_props::Properties, delegation::display::delegate_embed_display,
};
use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::{
		component::{
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
		.unwrap_or("Untitled Group".into());
	let description = description.and_then(|block| block.block_data);
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

	if let Some(description) = description {
		content.push(box TextComponent::new(&description))
	}
	content.push(stack);

	Ok(
		DisplayObject::new(box content)
			.meta(DisplayMeta::default().page(PageMeta::new().header(&name))),
	)
}
