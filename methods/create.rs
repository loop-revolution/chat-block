use block_tools::blocks::BlockType;
use block_tools::{
	blocks::Context,
	models::{Block, NewBlock},
	BlockError, LoopError,
};
use serde::{Deserialize, Serialize};

use crate::blocks::{chat_block::ChatBlock, data_block};

impl ChatBlock {
	pub fn handle_create_raw(
		input: String,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let input = serde_json::from_str::<CreationArgs>(&input);
		let input: CreationArgs = input.map_err(|_| BlockError::InputParse)?;

		Self::handle_create(input, context, user_id)
	}
}

impl ChatBlock {
	pub fn handle_create(
		input: CreationArgs,
		context: &Context,
		user_id: i32,
	) -> Result<Block, LoopError> {
		let conn = &context.conn()?;

		let block = NewBlock::new(Self::name(), user_id).insert(conn)?;

		if let Some(name) = input.name {
			let name_block = NewBlock {
				block_data: Some(name),
				..NewBlock::new(data_block::BLOCK_NAME, user_id)
			}
			.insert(conn)?;

			block.make_property("name", name_block.id).insert(conn)?;
		}

		Ok(block)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreationArgs {
	pub name: Option<String>,
}

impl Default for CreationArgs {
	fn default() -> Self {
		Self {
			name: Some("".into()),
		}
	}
}
