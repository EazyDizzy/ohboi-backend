pub async fn get_data(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url)
        .await?;

    Ok(resp.text().await?)
}