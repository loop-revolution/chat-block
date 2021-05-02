use std::time::SystemTime;

use block_tools::{
	auth::{optional_token, optional_validate_token},
	blocks::Context,
	display_api::component::{
		atomic::{icon::Icon, text::TextComponent},
		layout::{
			card::{CardComponent, CardHeader},
			stack::StackComponent,
		},
		menus::menu::MenuComponent,
		DisplayComponent,
	},
	models::Block,
	LoopError,
};

use crate::blocks::chat_block::ChatBlock;

impl ChatBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self { name, messages } = Self::from_id(block.id, user_id, conn)?;

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Chat".into());

		let mut content = StackComponent::vertical();
		content.push(TextComponent::new(format!("{} messages", messages.len())));
		let mut latest_time = "Never".to_string();
		if let Some(block) = messages.last() {
			let diff = SystemTime::now()
				.duration_since(block.created_at)
				.expect("Clock may have gone backwards");
			let mins = diff.as_secs() / 60;
			let minutes = mins % 60;
			let hours = (mins / 60) % 60;
			let days = ((mins / 24) / 60) / 60;
			latest_time = format!("{}d {}h {}m ago", days, hours, minutes);
		}
		content.push(TextComponent::new(format!(
			"Latest message: {}",
			latest_time
		)));

		let mut header = CardHeader {
			block_id: Some(block.id.to_string()),
			icon: Some(Icon::Message),
			..CardHeader::new(name)
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			menu.load_comments(conn)?;
			header.menu = Some(menu);
		}

		Ok(CardComponent {
			color: block.color.clone(),
			header: Some(box header),
			..CardComponent::new(content)
		}
		.into())
	}
}
