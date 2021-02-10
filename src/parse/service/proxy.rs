use std::fs::File;
use std::io;
use std::io::Write;

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use reqwest::Proxy;
use serde::{Deserialize, Serialize};

pub fn get_random_proxy() -> Proxy {
    let proxy_list = include_str!("../../../cache/proxies");
    let proxies = proxy_list.split("\n").collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, proxies.len() - 1);

    let selected_proxy = proxies[range.sample(&mut rng)].to_string();

    Proxy::all(&selected_proxy).unwrap()
}

pub async fn cache_list_of_proxies() -> Result<(), reqwest::Error> {
    let list = pull_list_of_proxies().await?;
    let proxies: Vec<String> =
        list.iter()
            .map(|p| {
                ["http://", p.ip.as_str(), ":", p.port.as_str()].join("")
            })
            .collect();
    let cache_result = cache_proxies(proxies);

    if cache_result.is_err() {
        log::error!("Caching failed: {:?}", cache_result.err());
    }

    Ok(())
}

fn cache_proxies(proxies: Vec<String>) -> Result<(), io::Error> {
    let proxies_string: String = proxies.join("\n");

    let mut file = File::create("./cache/proxies")?;
    file.write_all(proxies_string.as_bytes())?;

    Ok(())
}

async fn pull_list_of_proxies() -> Result<Vec<ProxySettings>, reqwest::Error> {
    let body = reqwest::get("http://pubproxy.com/api/proxy?user_agent=true&https=true&referer=true&limit=5&last_check=30&type=socks5&speed=10&level=anonymous")
        .await?
        .json::<PubProxyResponse>()
        .await?;

    Ok(body.data)
}

#[derive(Debug, Deserialize, Serialize)]
struct PubProxyResponse {
    data: Vec<ProxySettings>,
    count: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProxySettings {
    ip: String,
    port: String,
    country: String,
    last_checked: String,
    proxy_level: String,
    speed: String,
    support: ProxySupport,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProxySupport {
    https: u8,
    get: u8,
    post: u8,
    cookies: u8,
    referer: u8,
    user_agent: u8,
    google: u8,
}