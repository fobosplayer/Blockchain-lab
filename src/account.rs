use ed25519_dalek::ed25519::signature;
use uuid::Uuid;

use crate::{keypair::KeyPair, operation::Operation};

pub type Balance = i64;  // Unit: remol

pub struct Account {
	id: Uuid,
	wallet: Vec<KeyPair>,
	balance: Balance,
}

impl Account {
	pub fn new() -> Self {
		Self {
			id: Uuid::new_v4(),
			wallet: vec![],
			balance: 0
		}
	}

	pub fn id(&self) -> Uuid { self.id }

	pub fn balance(&self) -> Balance { self.balance }

	pub fn deposit(&mut self, amount: Balance) -> Result<(), String> {
		if amount <= 0 {
			return Err("Amount cannot be negative".to_string());
		}

		self.balance += amount;

		return Ok(());
	}

	pub fn withdraw(&mut self, amount: Balance) -> Result<(), String> {
		if amount <= 0 {
			return Err("Amount cannot be negative".to_string());
		}
		if self.balance < amount {
			return Err("Insufficient funds".to_string());
		}

		self.balance -= amount;

		return Ok(());
	}
	
	pub fn add_key_pair_to_wallet(&mut self, key_pair: KeyPair) {
		self.wallet.push(key_pair);
	}

	pub fn create_operation(&self, recipient_id: &Uuid, amount: Balance, signature_index: usize) -> Option<Operation> {
		self.wallet.get(signature_index).map(|key_pair| {
			let signature = key_pair.sign(format!("{}{}{}", self.id, recipient_id, amount).as_bytes());
			Operation::new(self.id, *recipient_id, amount, signature)
		})
	}

	pub fn sign(&self, data: &str, key_ind: usize) -> Option<&KeyPair> {
		return self.wallet.get(key_ind);
	}
}
