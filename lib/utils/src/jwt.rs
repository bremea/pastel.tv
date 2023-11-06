use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn sign_jwt(id: &String) -> String {
	//! replace secret
	//! fix this absolutely horrible code
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("id", id);
    return claims.sign_with_key(&key).unwrap();
}

#[derive(Clone)]
pub struct VerifyTokenResult {
    pub valid: bool,
    pub uuid: String,
}

pub fn verify_jwt(token_str: &String) -> VerifyTokenResult {
	//! replace secret
	//! fix this absolutely horrible code
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token_verify: Result<BTreeMap<String, String>, jwt::Error> =
        token_str.verify_with_key(&key);

    match token_verify {
        Ok(sk) => {
            let id: &String = &sk["id"];
            return VerifyTokenResult {
                valid: true,
                uuid: id.to_string(),
            };
        }
        Err(_) => {
            return VerifyTokenResult {
                valid: false,
                uuid: "".to_string(),
            };
        }
    }
}