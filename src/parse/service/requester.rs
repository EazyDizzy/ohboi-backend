use actix_web::http::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, REFERER};
use actix_web::web::Bytes;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use reqwest::Response;
use reqwest::header::USER_AGENT;

use crate::parse::service::proxy::get_random_proxy;

pub async fn get_data(url: String) -> Result<String, reqwest::Error> {
    let response = get_request(url).await?;
    let text = response.text().await?;
    println!("request complete");

    Ok(text)
}

pub async fn get_bytes(url: String) -> Result<Bytes, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?)
}

pub async fn get_request(url: String) -> Result<Response, reqwest::Error> {
    // TODO if proxy failed try another
    // TODO if all failed do without proxy
    let client = reqwest::Client::builder()
        .gzip(true)
        .brotli(true)
        .proxy(get_random_proxy())
        .build().unwrap();

    let req = client
        .get(url.as_str())
        .header(USER_AGENT, get_random_user_agent())
        .header(REFERER, get_random_referer())
        .header(ACCEPT_LANGUAGE, "en-gb")
        .header(ACCEPT_ENCODING, "*")
        .header(ACCEPT, "*/*")
        .header(CONNECTION, "keep-alive")
        ;
    // TODO proxy, find 5 random proxies every 10 minutes, check their health
    req
        .send()
        .await
}

fn get_random_referer() -> String {
    let referer_list = include_str!("../../../cache/referrers");
    let referrers = referer_list.split("\n").collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, referrers.len() - 1);

    referrers[range.sample(&mut rng)].to_string()
}

fn get_random_user_agent() -> String {
    let site_list = include_str!("../../../cache/user_agents");
    let sites = site_list.split("\n").collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, sites.len() - 1);

    sites[range.sample(&mut rng)].to_string()
}