use openssl::sign::{Signer, Verifier};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

pub fn sign(data: &[u8], keypair: &PKey) -> Vec<u8> {
    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(data).unwrap();
    signer.finish().unwrap()
}

pub fn verify(data: &[u8], signature: &[u8], keypair: &PKey) -> bool {
    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(data).unwrap();
    verifier.finish(signature).unwrap()
}
