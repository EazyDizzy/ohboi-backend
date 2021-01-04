use actix_web::web::Bytes;

pub async fn get_data(url: String) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url.as_str())
        .await?;

    Ok(resp.text().await?)
}

pub async fn get_bytes(url: &str) -> Result<Bytes, reqwest::Error> {
    let resp = reqwest::get(url)
        .await?;

    Ok(resp.bytes().await?)
}