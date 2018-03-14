use signatory::ed25519::PublicKey as SignatoryKey;
pub use signatory::ed25519::PUBLIC_KEY_SIZE;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PublicKey(SignatoryKey);

impl PublicKey {
    /// Obtain public key as a byte array reference
    #[inline]
    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_SIZE] {
        self.0.as_bytes()
    }

    /// Convert public key into owned byte array
    #[inline]
    pub fn into_bytes(self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0.into_bytes()
    }
}

impl From<SignatoryKey> for PublicKey {
    fn from(key: SignatoryKey) -> PublicKey {
        PublicKey(key)
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state);
    }
}
