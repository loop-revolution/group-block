use block_tools::{blocks::Context, models::Block, Error};
use serde::{Deserialize, Serialize};

pub fn create(_input: String, _context: &Context, _user_id: i32) -> Result<Block, Error> {
	Err(Error::GenericError)
}

#[derive(Serialize, Deserialize, Debug)]
struct CreationArgs {
	name: String,
	content: String,
}
