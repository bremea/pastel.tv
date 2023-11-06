use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use utils::jwt::verify_jwt;

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
