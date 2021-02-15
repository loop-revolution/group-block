use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::component::{
		card::{CardComponent, CardHeader, Icon},
		stack::{StackComponent, StackDirection},
		text::TextComponent,
		DisplayComponent, WrappedComponent,
	},
	models::Block,
	Error,
};

use crate::{
	blocks::group_block::group_props::Properties, delegation::display::delegate_embed_display,
};

pub fn embed_display(block: &Block, context: &Context) -> Result<Box<dyn DisplayComponent>, Error> {
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
	let description = description.and_then(|block| block.block_data);
	let items: Vec<WrappedComponent> = items
		.iter()
		.map(|block| WrappedComponent::from(delegate_embed_display(block, context)))
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

	Ok(box CardComponent {
		color: None,
		content: box content,
		header: CardHeader::new(&name).id(block.id).icon(Icon::Folder),
	})
}
