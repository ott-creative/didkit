use crate::config::env::API_VERSION;
use crate::response::{ApiSuccess, Success};
use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    dto::{LoginInput, RegisterInput, TokenPayload, VcTpltInput},
    error::{ApiResult, Error},
    model::{User, VcTplt},
    service::{did::DidService, user::AuthService, vc::VcTpltService},
    utils::{jwt, validate_payload},
};

/// Generate DID JWK for user, return pub key
/// TODO: user auth
pub async fn did_create() -> ApiResult<Json<ApiSuccess<String>>> {
    let did = DidService::did_create().await?;
    Ok(Json(ApiSuccess {
        api_version: API_VERSION.to_string(),
        body: Success { data: did },
    }))
}
