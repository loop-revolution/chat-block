use crate::blocks::{chat_block::ChatBlock, data_block::DataBlock, text_block::TextBlock};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			form::input::{InputComponent, InputSize},
			layout::messagelist::{MessageListComponent, MessageListMessage},
			menus::menu::MenuComponent,
			misc::richtext::RichTextComponent,
		},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::{Block, User},
	LoopError,
};

impl ChatBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;
		let user = if let Some(id) = user_id {
			User::by_id(id, conn)?
		} else {
			None
		};

		// Get all the blocks properties
		let Self {
			name,
			messages: message_blocks,
		} = Self::from_id(block.id, user_id, conn)?;

		let name_string = name.clone().and_then(|block| block.block_data);
		let mut messages: Vec<MessageListMessage> = vec![];

		for message in message_blocks {
			let owner =
				User::by_id(message.owner_id, conn)?.expect("User didn't exist for user id");
			let content = TextBlock::data_to_display(&message.block_data.unwrap_or_default());
			let component = RichTextComponent {
				content,
				..Default::default()
			};
			let message = MessageListMessage {
				sent_at: message.created_at.into(),
				stars: Some(message.stars.len() as i32),
				..MessageListMessage::new(
					component,
					owner.display_name.unwrap_or(owner.username.clone()),
					owner.username,
				)
			};
			messages.push(message);
		}

		let messagelist = MessageListComponent {
			color: block.color.clone(),
			input_name: Some("MESSAGE_TEXT".to_string()),
			input_placeholder: Some("Write a message...".to_string()),
			send_method: Some(Self::send_method_object(block.id)),
			messages,
			..Default::default()
		};

		let mut page = PageMeta::default();
		let header_backup = name_string.unwrap_or_else(|| "Untitled Chat".into());

		if let Some(user) = user {
			let mut menu = MenuComponent::from_block(block, user.id);
			menu.load_comments(conn)?;
			page.menu = Some(menu);
			if let Some(name) = name {
				if has_perm_level(user.id, &name, PermLevel::Edit) {
					page.header_component = Some(
						InputComponent {
							label: Some("Chat Name".into()),
							size: Some(InputSize::Medium),
							..DataBlock::masked_editable_data(
								name.id.to_string(),
								name.block_data,
								true,
							)
						}
						.into(),
					);
				} else {
					page.header = Some(header_backup)
				}
			}
		} else {
			page.header = Some(header_backup)
		}

		let meta = DisplayMeta {
			page: Some(page),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(messagelist)
		})
	}
}
