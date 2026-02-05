use serde::de::DeserializeOwned;

use crate::OxideError;

fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{base}/{path}")
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_json<T>(path: &str) -> Result<T, OxideError>
where
    T: DeserializeOwned,
{
    let base_url = crate::api::bridge::api_base_url();
    let url = join_url(&base_url, path);

    let client = reqwest::Client::new();
    client
        .get(url)
        .header(reqwest::header::USER_AGENT, "oxide-api-browser-example")
        .send()
        .await
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })?
        .error_for_status()
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })?
        .json::<T>()
        .await
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub async fn get_json<T>(path: &str) -> Result<T, OxideError>
where
    T: DeserializeOwned,
{
    let base_url = crate::api::bridge::api_base_url();
    let url = join_url(&base_url, path);

    let client = reqwest::Client::new();
    client
        .get(url)
        .send()
        .await
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })?
        .error_for_status()
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })?
        .json::<T>()
        .await
        .map_err(|e| OxideError::Internal {
            message: e.to_string(),
        })
}

#[cfg(all(target_arch = "wasm32", not(target_os = "unknown")))]
pub async fn get_json<T>(_path: &str) -> Result<T, OxideError>
where
    T: DeserializeOwned,
{
    Err(OxideError::Validation {
        message: "HTTP client not available for this WebAssembly target".to_string(),
    })
}

