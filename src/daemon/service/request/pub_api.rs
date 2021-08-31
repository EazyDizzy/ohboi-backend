use crate::daemon::service::request::layer::get_request;

pub async fn get_data_s(url: String) -> Result<String, reqwest::Error> {
    get_data(&url).await
}

pub async fn get_data(url: &str) -> Result<String, reqwest::Error> {
    let response = get_request(url).await?;

    let text = response.text().await?;

    Ok(text)
}

pub async fn get_bytes(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?.to_vec())
}
