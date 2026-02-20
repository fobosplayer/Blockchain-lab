use ed25519_dalek::{Signature, SignatureError, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

/// Pair of private and public keys.
pub struct KeyPair {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let mut csprng = OsRng;

        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Self {
            private_key: signing_key,
            public_key: verifying_key,
        }
    }

    /// Uses the private key to sign the passed message.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.private_key.sign(message)
    }

    /// Uses the public key to verify the message.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.public_key.verify(message, signature)
    }
}
