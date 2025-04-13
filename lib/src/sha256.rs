use primitive_types::U256;
// use core::fmt;
use std::fmt;
use sha256::digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);

impl Hash {
    pub fn hash<T: serde::Serialize> (data: &T) -> Self {

        // hash anything that can be serde serialized via ciborium
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized,) {
            panic!(
                "Failed to serialize data: {:?}. \
                This should not happen",
                e
            );
        }
        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice()
            .try_into()
            .unwrap();
        Hash(U256::from(hash_array))
    }

    // check if a hash matches a target

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
    // Zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
    impl fmt::Display for Hash {
        fn fmt(&self _: fmt::Formatter) -> fmt::Result {
            write!(f, "{:x}", self.0)
        }
    }


}