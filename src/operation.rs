use ed25519_dalek::Signature;
use uuid::Uuid;

pub struct Operation {
	sender_id: Uuid,
	receiver_id: Uuid,
	amount: i64,
	signature: Signature,
}

impl Operation {
	pub fn new(sender_id: Uuid, receiver_id: Uuid, amount: i64, signature: Signature) -> Self {
		Self {
			sender_id: sender_id,
			receiver_id: receiver_id,
			amount: amount,
			signature: signature
		}
	}

	fn verify() -> bool {
		true
	}
}
