use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::component::{
		atomic::{icon::Icon, text::TextComponent},
		layout::{
			card::{CardComponent, CardHeader},
			stack::StackComponent,
		},
		menus::menu::{CustomMenuItem, MenuComponent},
		DisplayComponent, WrappedComponent,
	},
	models::Block,
	LoopError,
};

use crate::{blocks::group_block::GroupBlock, delegation::display::delegate_embed_display};

impl GroupBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self {
			name,
			description,
			items,
		} = Self::from_id(block.id, user_id, conn)?;

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Group".into());
		let description = description.and_then(|block| block.block_data);
		let items: Vec<WrappedComponent> = items
			.iter()
			.map(|block| WrappedComponent::from(delegate_embed_display(block, context)))
			.collect();

		let stack: DisplayComponent = if items.is_empty() {
			TextComponent::info("No items in group").into()
		} else {
			StackComponent {
				items,
				..Default::default()
			}
			.into()
		};
		let mut content = StackComponent::vertical();

		if let Some(description) = description {
			content.push(TextComponent::new(description).into())
		}
		content.push(stack);

		let mut header = CardHeader {
			block_id: Some(block.id.to_string()),
			icon: Some(Icon::Folder),
			..CardHeader::new(name)
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let action = Self::build_add_action_object(block.id);
				let mut item = CustomMenuItem::new("Add a Block", Icon::Plus);
				item.interact = Some(action);
				menu.custom = Some(vec![item]);
			}
			header.menu = Some(menu);
		}

		Ok(CardComponent {
			color: block.color.clone(),
			content: box content.into(),
			header: Some(box header),
		}
		.into())
	}
}
