use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent, form::input::InputComponent, layout::stack::StackComponent,
		},
		CreationObject,
	},
	Error,
};

use crate::blocks::group_block::GroupBlock;
impl GroupBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, Error> {
		let header = TextComponent::heading("New Group Block");

		let name_input = InputComponent {
			label: Some("Name".to_string()),
			name: Some("NAME".to_string()),
			..InputComponent::default()
		};
		let desc_input = InputComponent {
			label: Some("Description".to_string()),
			name: Some("DESC".to_string()),
			..Default::default()
		};
		let items_input = TextComponent::info("You will be able to add blocks after creation.");

		let mut main = StackComponent::vertical();
		main.push(name_input.into());
		main.push(desc_input.into());
		main.push(items_input.into());

		let template: String = r#"{
			"name": $[NAME]$,
			"desc": $[DESC]$,
			"items": []
		}"#
		.split_whitespace()
		.collect();
		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: template,
		};
		Ok(object)
	}
}
