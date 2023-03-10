use leptos::*;
use web_sys::window;

pub async fn fetch_api<T>(_cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let mut base_url: String = "http://localhost:8080".to_string();
    if let Some(window) = window() {
        let location = window.location();
        if let Ok(origin) = location.origin() {
            if origin != "http://localhost:8080" {
                base_url = origin;
            }
        }
    }
    let full_path = format!("{}{}", base_url.to_string(), path.to_string());
    let json = reqwest::get(&full_path)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::from_json(&json).map_err(|e| log::error!("{e}")).ok()
}
