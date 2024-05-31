use std::ops::Deref;

use actix_web::http::header::HeaderMap;

use actix_web::HttpMessage;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use actix_web_lab::middleware::Next;
use anyhow::Context;
use base64::prelude::*;
use secrecy::Secret;
use uuid::Uuid;

use super::token::get_claim_from_token;
use super::Credentials;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let (http_request, payload) = req.parts_mut();

    // println!("### reject_anonymous_users >>>");
    match get_user_id_from_auth_token(http_request.headers()) {
        Ok(user_id) => {
            // println!("### reject_anonymous_users, 1. user_id = {:?}", user_id);
            match user_id {
                Some(user_id) => {
                    // println!("### reject_anonymous_users, 2. user_id = {:?}", user_id);
                    req.extensions_mut().insert(UserId(user_id));
                    next.call(req).await
                }
                None => {
                    // println!("### reject_anonymous_users, 3");
                    Err(actix_web::error::ErrorUnauthorized("Authentication failed"))
                }
            }
        }
        Err(err) => {
            // println!("### reject_anonymous_users, 4");
            Err(actix_web::error::ErrorUnauthorized(err))
        }
    }
}

pub fn basic_authentication(headers: &HeaderMap) -> Result<Option<Credentials>, anyhow::Error> {
    let header_value = match headers.get("Authorization") {
        None => return Ok(None),
        Some(header_value) => header_value,
    };

    let header_value = header_value
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not 'Basic'.")?;
    let decoded_bytes = BASE64_STANDARD
        .decode(base64encoded_segment)
        .context("Failed to base64-decode 'Basic' credentials.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8.")?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Some(Credentials {
        username,
        password: Secret::new(password),
    }))
}

fn get_user_id_from_auth_token(headers: &HeaderMap) -> Result<Option<Uuid>, anyhow::Error> {
    // println!("### get_user_id_from_auth_token >>>");
    let header_value = match headers.get("Authorization") {
        None => return Ok(None),
        Some(header_value) => header_value,
    };
    // println!("### get_user_id_from_auth_token: header_value = {:?}", header_value);

    let header_value = header_value
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;
    // println!("### get_user_id_from_auth_token: header_value = {:?}", header_value);
    let token_segment = header_value
        .strip_prefix("Bearer ")
        .context("The authorization scheme was not 'Bearer'.")?;
    // println!("### get_user_id_from_auth_token: token_segment = {:?}", token_segment);

    let user_id_in_string = get_claim_from_token(&token_segment)?;
    // println!("### get_user_id_from_auth_token: user_id_in_string = {:?}", user_id_in_string);

    let user_id = Uuid::parse_str(&user_id_in_string)?;
    // println!("### get_user_id_from_auth_token: user_id = {:?}", user_id);

    Ok(Some(user_id))
}
