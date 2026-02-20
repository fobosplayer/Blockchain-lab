use uuid::Uuid;

use crate::{operation::Operation};

pub struct Transaction {
	id: Uuid,
	operations: Vec<Operation>,
	nonce: i32,
}

impl Transaction {
	pub fn new(operations: Vec<Operation>, nonce: i32) -> Self {
		Self {
			id: Uuid::new_v4(),
			operations: operations,
			nonce: nonce,
		}
	}
}
