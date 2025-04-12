use ecdsa::{
    signature::Signer,
Signature as ECDSASignature,
SigningKey,
VerifyingKey
};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq
)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(
    #[serde(with = "signkey_serde")]
    pub SigningKey<Secp256k1>
);
