use serde::de::DeserializeOwned;

pub async fn get<T>(url: &str) -> Option<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let request = gloo_net::http::Request::get(&url).send().await;
    match request {
        Ok(response) => {
            if response.ok() {
                match response.json().await {
                    Ok(result) => Some(result),
                    Err(error) => {
                        log::error!("Error deserializing {}:\n{}", url, error);
                        None
                    },
                }
            } else {
                log::error!("Error fetching {}: {} ({})", url, response.status(), response.status_text());
                None
            }
        }
        Err(error) => {
            log::error!("Failed to fetch {}:\n{}", url, error);
            None
        }
    }
}

pub async fn post<T>(url: String, body: T) -> Result<u16, u16>
where
    T: serde::Serialize,
{
    let request = gloo_net::http::Request::post(&url)
        .json(&body).or(Err(400_u16))?.send().await;
    match request {
        Ok(response) => {
            if response.ok() {
                Ok(response.status())
            } else {
                Err(response.status())
            }
        }
        Err(_) => {
            Err(400_u16)
        }
    }
}
