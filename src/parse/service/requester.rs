use actix_web::web::Bytes;
use reqwest::Response;

pub async fn get_data_s(url: String) -> Result<String, reqwest::Error> {
    get_data(&url).await
}

pub async fn get_data(url: &str) -> Result<String, reqwest::Error> {
    let response = get_request(url).await?;

    let text = response.text().await?;

    Ok(text)
}

pub async fn get_bytes(url: &str) -> Result<Bytes, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?)
}

pub async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    // TODO preconnect, zip header
    // TODO random headers
    // TODO proxy

    reqwest::Client::new()
        .get(url)
        .send()
        .await
}