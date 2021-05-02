use crate::blocks::{
	chat_block::{ChatBlock, BLOCK_NAME},
	text_block::{self, data_convert::ComponentStruct},
};
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	blocks::Context,
	display_api::{component::DisplayComponent, MethodObject},
	models::Block,
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

impl ChatBlock {
	pub fn send_method(context: &Context, block_id: i64, args: String) -> Result<Block, LoopError> {
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;

		let access_err: LoopError =
			BlockError::TypeGenericError(format!("Cannot send blocks to {}", block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(b) => b,
			None => return Err(access_err),
		};
		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}
		let invalid_err: LoopError = BlockError::InputParse.into();
		let input = match serde_json::from_str::<SendArgs>(&args) {
			Ok(input) => input,
			Err(_) => return Err(invalid_err),
		};
		let display_vec: Vec<DisplayComponent> = input
			.text
			.into_iter()
			.map(|component| component.args.into())
			.collect();
		let new_message_block =
			text_block::TextBlock::handle_create_vec(display_vec, context, user_id)?;
		block
			.make_property("message", new_message_block.id)
			.insert(conn)?;
		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct SendArgs {
	text: Vec<ComponentStruct>,
}

impl ChatBlock {
	pub fn send_method_object(block_id: i64) -> MethodObject {
		MethodObject {
			block_type: BLOCK_NAME.into(),
			block_id: block_id.to_string(),
			method_name: "send".to_string(),
			arg_template: r#"{"text":$[MESSAGE_TEXT]$}"#.into(),
		}
	}
}
