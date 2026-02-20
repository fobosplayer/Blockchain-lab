use uuid::Uuid;

use crate::transaction::{self, Transaction};

pub struct Block {
	id: Uuid,
	previous_hash: String,
	transactions: Vec<Transaction>,
}

impl Block {
	pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
		Self {
			id: Uuid::new_v4(),
			transactions: transactions,
			previous_hash: previous_hash,
		}
	}
}
