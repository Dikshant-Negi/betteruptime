use poem::{
    Endpoint, Middleware, Request, Result, Error,
    http::StatusCode,
};

use crate::extra::jwt::Claims;

pub struct TokenMiddleware;

impl<E: Endpoint> Middleware<E> for TokenMiddleware {
    type Output = TokenMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TokenMiddlewareImpl { ep }
    }
}

pub struct TokenMiddlewareImpl<E> {
    ep: E,
}

#[derive(Clone)]
pub struct Token {
    pub user_id: String,
}

impl<E: Endpoint> Endpoint for TokenMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // Extract header
        let auth_header = match req.header("Authorization") {
            Some(h) => h,
            None => {
                return Err(Error::from_status(StatusCode::UNAUTHORIZED));
            }
        };

        // Must be "Bearer <token>"
        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t,
            None => {
                return Err(Error::from_status(StatusCode::UNAUTHORIZED));
            }
        };

        // Decode JWT
        let decoded = match Claims::decode_token(token.to_string()) {
            Ok(c) => c,
            Err(_) => {
                return Err(Error::from_status(StatusCode::UNAUTHORIZED));
            }
        };

        // Insert user_id into request
        req.extensions_mut().insert(Token {
            user_id: decoded.sub,
        });

        // Call next endpoint
        self.ep.call(req).await
    }
}
