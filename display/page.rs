use crate::{
	blocks::{
		data_block::masked_data_edit,
		group_block::{group_props::Properties, methods::add::add_action_object},
	},
	delegation::display::delegate_embed_display,
};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			icon::Icon,
			input::InputSize,
			menu::{CustomMenuItem, MenuComponent},
			stack::{StackComponent, StackDirection},
			text::TextComponent,
			DisplayComponent, WrappedComponent,
		},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::{Block, User},
	Error,
};

pub fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error> {
	let conn = &context.conn()?;
	let user_id = optional_validate_token(optional_token(context))?;
	let user = if let Some(id) = user_id {
		User::by_id(id, conn)?
	} else {
		None
	};
	let is_root = match &user {
		None => false,
		Some(user) => user.root_id == Some(block.id),
	};

	// Get all the blocks properties
	let Properties {
		name,
		description,
		items,
	} = Properties::build(block.id, user_id, conn)?;

	let name_string = name.clone().and_then(|block| block.block_data);
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

	if !is_root {
		if let Some(desc) = desc {
			let block = description.unwrap();
			content.push(
				box masked_data_edit(block.id.to_string(), Some(desc), false)
					.label("Description")
					.size(InputSize::MultiLine),
			)
		}
	}
	content.push(stack);

	let mut page = PageMeta::new();
	let header_backup = name_string.unwrap_or_else(|| "Untitled Group".into());

	if let Some(user) = user {
		page.menu = Some(MenuComponent::load_from_block(block, user.id));
		if !is_root {
			if let Some(name) = name {
				if has_perm_level(user.id, &name, PermLevel::Edit) {
					page = page.header_component(
						box masked_data_edit(name.id.to_string(), name.block_data, true)
							.label("Group Name")
							.size(InputSize::Medium),
					)
				} else {
					page = page.header(&header_backup)
				}
			}
		}
		if let Some(mut menu) = page.menu.clone() {
			if has_perm_level(user.id, &block, PermLevel::Edit) {
				let action = add_action_object(block.id);
				let item = CustomMenuItem::new("Add a Block", Icon::Plus).interact(action);
				menu.custom = Some(vec![item]);
				page.menu = Some(menu)
			}
		}
	} else {
		page = page.header(&header_backup)
	}

	Ok(DisplayObject::new(box content).meta(DisplayMeta::default().page(page)))
}
