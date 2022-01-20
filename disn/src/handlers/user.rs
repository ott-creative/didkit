use crate::config::env::API_VERSION;
use crate::response::{ApiSuccess, Success};
use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    dto::{LoginInput, RegisterInput, TokenPayload, VcTpltInput},
    error::{ApiResult, Error},
    model::{User, VcTplt},
    service::{user::AuthService, vc::VcTpltService},
    utils::{jwt, validate_payload},
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

pub async fn echo() -> ApiResult<Json<ApiSuccess<String>>> {
    Ok(Json(ApiSuccess {
        api_version: API_VERSION.to_string(),
        body: Success {
            data: "hello".to_string(),
        },
    }))
}

pub async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<ApiSuccess<TokenPayload>>> {
    validate_payload(&input)?;
    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;
    Ok(Json(ApiSuccess {
        api_version: API_VERSION.to_string(),
        body: Success {
            data: TokenPayload {
                access_token: token,
                token_type: BEARER.to_string(),
            },
        },
    }))
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<ApiSuccess<TokenPayload>>)> {
    validate_payload(&input)?;
    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id)?;
    Ok((
        StatusCode::CREATED,
        Json(ApiSuccess {
            api_version: API_VERSION.to_string(),
            body: Success {
                data: TokenPayload {
                    access_token: token,
                    token_type: BEARER.to_string(),
                },
            },
        }),
    ))
}