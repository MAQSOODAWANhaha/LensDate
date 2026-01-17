use axum::{
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[derive(serde::Serialize)]
struct UploadResp {
    file_name: String,
    file_url: String,
    size: usize,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(upload_file))
        .route("/:name", get(get_file))
}

async fn upload_file(
    AuthUser { .. }: AuthUser,
    axum::extract::State(_state): axum::extract::State<AppState>,
    mut multipart: Multipart,
) -> ApiResult<UploadResp> {
    let upload_dir = upload_dir();
    fs::create_dir_all(&upload_dir).await.map_err(|_| ApiError::internal())?;

    while let Some(field) = multipart.next_field().await.map_err(|_| ApiError::bad_request("invalid_multipart"))? {
        if field.name() != Some("file") {
            continue;
        }

        let file_name = field.file_name().map(|name| name.to_string()).unwrap_or_default();
        let ext = std::path::Path::new(&file_name)
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or("bin");
        if !is_allowed_type(field.content_type().map(|v| v.to_string()), ext) {
            return Err(ApiError::bad_request("invalid_file_type"));
        }
        let stored_name = format!("{}.{}", Uuid::new_v4(), ext);
        let file_path = upload_dir.join(&stored_name);

        let data = field.bytes().await.map_err(|_| ApiError::bad_request("invalid_file"))?;
        let max_bytes = max_upload_bytes();
        if data.len() > max_bytes {
            return Err(ApiError::bad_request("file_too_large"));
        }
        let mut file = fs::File::create(&file_path).await.map_err(|_| ApiError::internal())?;
        file.write_all(&data).await.map_err(|_| ApiError::internal())?;

        let file_url = format!("/uploads/{}", stored_name);
        return Ok(Json(crate::common::ApiResponse::ok(UploadResp {
            file_name: stored_name,
            file_url,
            size: data.len(),
        })));
    }

    Err(ApiError::bad_request("file_required"))
}

async fn get_file(
    axum::extract::State(_state): axum::extract::State<AppState>,
    Path(name): Path<String>,
) -> Result<Response, ApiError> {
    let upload_dir = upload_dir();
    let file_path = upload_dir.join(&name);
    let data = fs::read(&file_path).await.map_err(|_| ApiError::not_found())?;
    let content_type = content_type_for(&name);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(axum::body::Body::from(data))
        .map_err(|_| ApiError::internal())?;
    Ok(response)
}

fn upload_dir() -> std::path::PathBuf {
    std::env::var("UPLOAD_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/tmp/uploads"))
}

fn content_type_for(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else {
        "application/octet-stream"
    }
}

fn max_upload_bytes() -> usize {
    std::env::var("UPLOAD_MAX_BYTES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10 * 1024 * 1024)
}

fn is_allowed_type(content_type: Option<String>, ext: &str) -> bool {
    let ext = ext.to_lowercase();
    let allowed_ext = ["png", "jpg", "jpeg", "gif", "webp", "pdf"];
    let allowed_ct = [
        "image/png",
        "image/jpeg",
        "image/gif",
        "image/webp",
        "application/pdf",
    ];
    if let Some(ct) = content_type
        && allowed_ct.contains(&ct.as_str())
    {
        return true;
    }
    allowed_ext.contains(&ext.as_str())
}
