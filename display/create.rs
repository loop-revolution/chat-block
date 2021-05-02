use block_tools::{
	blocks::Context,
	display_api::{
		component::{
			atomic::text::TextComponent, form::input::InputComponent, layout::stack::StackComponent,
		},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::chat_block::ChatBlock;

impl ChatBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, LoopError> {
		let header = InputComponent {
			label: Some("Name".to_string()),
			name: Some("NAME".to_string()),
			..InputComponent::default()
		};
		let people_invite = TextComponent::info(
			"You'll be able to add users by using the permissions panel after creation.",
		);

		let mut main = StackComponent::vertical();
		main.push(people_invite);

		let template = r#"{ "name": $[NAME]$ }"#.to_string();
		let object = CreationObject {
			header_component: header.into(),
			main_component: main.into(),
			input_template: template,
		};
		Ok(object)
	}
}
