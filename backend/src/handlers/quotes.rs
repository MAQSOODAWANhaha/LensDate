use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::quotes::{
    AcceptQuoteResp, CreateQuoteReq, MyQuoteListItem, MyQuoteListQuery, QuoteDetailResp,
    QuoteListItem, QuoteListQuery, QuoteResp, QuoteVersionItem, UpdateQuoteReq,
};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::quotes_service;
use crate::state::AppState;

pub async fn list_quotes(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<QuoteListQuery>,
) -> ApiResult<Paged<QuoteListItem>> {
    let data = quotes_service::list_quotes(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_my_quotes(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<MyQuoteListQuery>,
) -> ApiResult<Paged<MyQuoteListItem>> {
    let data = quotes_service::list_my_quotes(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_quote(
    AuthUser { user_id }: AuthUser,
    Path(quote_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<QuoteDetailResp> {
    let data = quotes_service::get_quote(&state, user_id, quote_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update_quote(
    AuthUser { user_id }: AuthUser,
    Path(quote_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<UpdateQuoteReq>,
) -> ApiResult<QuoteResp> {
    let data = quotes_service::update_quote(&state, user_id, quote_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_quote_versions(
    AuthUser { user_id }: AuthUser,
    Path(quote_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<QuoteVersionItem>> {
    let data = quotes_service::list_quote_versions(&state, user_id, quote_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn withdraw_quote(
    AuthUser { user_id }: AuthUser,
    Path(quote_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<QuoteResp> {
    let data = quotes_service::withdraw_quote(&state, user_id, quote_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_quote(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateQuoteReq>,
) -> ApiResult<QuoteResp> {
    let data = quotes_service::create_quote(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn accept_quote(
    AuthUser { user_id }: AuthUser,
    Path(quote_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<AcceptQuoteResp> {
    let data = quotes_service::accept_quote(&state, user_id, quote_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
