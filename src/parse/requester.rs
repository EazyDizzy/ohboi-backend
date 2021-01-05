use actix_web::web::Bytes;
use reqwest::Response;

pub async fn get_data(url: String) -> Result<String, reqwest::Error> {
    let response = get_request(url).await?;

    // println!("{:?}", response.headers());

    let text = response.text().await?;

    Ok(text)
}

pub async fn get_bytes(url: String) -> Result<Bytes, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?)
}

pub async fn get_request(url: String) -> Result<Response, reqwest::Error> {
    // TODO preconnect, zip header

    reqwest::Client::new()
        .get(url.as_str())
        .send()
        .await
}