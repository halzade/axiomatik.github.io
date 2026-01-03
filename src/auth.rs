use rsa::{Pkcs1v15Sign, RsaPublicKey};
use rsa::pkcs8::DecodePublicKey;
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use std::fs;

pub fn verify_token(token: &str) -> bool {
    let public_key_pem = match fs::read_to_string("public_key.pem") {
        Ok(key) => key,
        Err(_) => return false,
    };

    let public_key = match RsaPublicKey::from_public_key_pem(&public_key_pem) {
        Ok(key) => key,
        Err(_) => return false,
    };

    let signature_bytes = match general_purpose::STANDARD.decode(token.trim()) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let message = b"Axiomatik Article Creation";
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed = hasher.finalize();

    let verifier = Pkcs1v15Sign::new::<Sha256>();
    public_key.verify(verifier, &hashed, &signature_bytes).is_ok()
}
