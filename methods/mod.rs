use super::ChatBlock;
use block_tools::{blocks::Context, models::Block, BlockError, LoopError};
pub mod create;
mod send;
pub mod visibility_update;
use block_tools::blocks::BlockType;

impl ChatBlock {
	pub fn handle_method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		match name.as_str() {
			"send" => Self::send_method(context, block_id, args),
			_ => Err(BlockError::MethodExist(name, Self::name()).into()),
		}
	}
}
