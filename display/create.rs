use block_tools::{
	blocks::Context,
	display_api::{component::text::TextComponent, CreationObject},
	Error,
};

pub fn create_display(_context: &Context, _user_id: i32) -> Result<CreationObject, Error> {
	Ok(CreationObject {
		header_component: box TextComponent::new("New Group Block"),
		main_component: box TextComponent::new("Coming soon"),
		input_template: String::default(),
	})
}
