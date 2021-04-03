use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			input::InputComponent,
			stack::{StackComponent, StackDirection},
			text::{TextComponent, TextPreset},
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
		let header = TextComponent::new("New Group Block").preset(TextPreset::Heading);
		let name_input = InputComponent::new().label("Name").name("NAME");
		let content_input = InputComponent::new().label("Description").name("DESC");
		let items_input = TextComponent::new("You will be able to add blocks after.");
		let main = StackComponent::new(StackDirection::Vertical)
			.append(box name_input)
			.append(box content_input)
			.append(box items_input);

		let template: String = r#"{
			"name": $[NAME]$,
			"desc": $[DESC]$,
			"items": []
		}"#
		.split_whitespace()
		.collect();
		let object = CreationObject {
			header_component: box header,
			main_component: box main,
			input_template: template,
		};
		Ok(object)
	}
}
