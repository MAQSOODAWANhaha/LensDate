use crate::dto::configs::{ConfigResp, UpdateConfigReq};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::configs_repo;
use crate::state::AppState;

pub async fn get_config(
    state: &AppState,
    key: String,
) -> ServiceResult<ConfigResp> {
    let model = configs_repo::find_config_by_key(&state.orm, &key)
        .await?
        .ok_or(DomainError::NotFound)?;

    Ok(ConfigResp {
        id: model.id,
        key: model.key,
        value: model.value,
    })
}

pub async fn upsert_config(
    state: &AppState,
    key: String,
    req: UpdateConfigReq,
) -> ServiceResult<ConfigResp> {
    let existing = configs_repo::find_config_by_key(&state.orm, &key).await?;

    let saved = match existing {
        Some(model) => configs_repo::update_config(&state.orm, model, req.value).await?,
        None => configs_repo::create_config(&state.orm, key.clone(), req.value).await?,
    };

    Ok(ConfigResp {
        id: saved.id,
        key: saved.key,
        value: saved.value,
    })
}
