use std::{collections::{HashMap, hash_map}, hash::Hash};

use crate::{account::Account, block::Block, transaction::Transaction};

pub struct Blockchain {
	coin_db: HashMap<Account, i64>,
	blocks: Vec<Block>,
	transactions: Vec<Transaction>,
	faucet_coins: i64,
}

impl Blockchain {
	pub fn new() -> Self {
		let genesis_block = Block::new(vec![], String::from("0"));

		Self {
			coin_db: HashMap::new(),
			blocks: vec![genesis_block],
			transactions: vec![],
			faucet_coins: 0,
		}
	}

	pub fn validate_block(block: Block) -> bool {
		true
	}

	pub fn get_token_from_faucet(account: Account, amount: i64) {
		
	}
}
