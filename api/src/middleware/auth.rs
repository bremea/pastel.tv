use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
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
    pub id: String,
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
                id: id.to_string(),
            };
        }
        Err(_) => {
            return VerifyTokenResult {
                valid: false,
                id: "".to_string(),
            };
        }
    }
}

pub async fn auth_middleware<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
where
    B: Send,
{
    let (mut parts, body) = request.into_parts();

    let auth: TypedHeader<Authorization<Bearer>> = parts
        .extract()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

	let verification = verify_jwt(&auth.token().to_string());

    if !verification.valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut request = Request::from_parts(parts, body);
	request.extensions_mut().insert(verification);
    Ok(next.run(request).await)
}
