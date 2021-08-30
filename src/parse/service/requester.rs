use rand::distributions::Uniform;
use rand::prelude::Distribution;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, REFERER, USER_AGENT};
use reqwest::Response;

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

async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    // TODO preconnect, zip header
    // TODO random headers
    // TODO proxy

    let client = reqwest::Client::builder().build().unwrap();

    let req = client
        .get(url)
        .header(USER_AGENT, get_random_user_agent())
        .header(REFERER, get_random_referer())
        .header(ACCEPT_LANGUAGE, "en-gb")
        .header(ACCEPT_ENCODING, "*")
        .header(ACCEPT, "*/*")
        .header(CONNECTION, "keep-alive")
        ;

    req
        .send()
        .await
}

static REFERER_LIST: &str = include_str!("../../../cache/referrers");
static USER_AGENT_LIST: &str = include_str!("../../../cache/user_agents");

fn get_random_referer() -> String {
    let referrers = REFERER_LIST.split('\n').collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, referrers.len() - 1);

    referrers[range.sample(&mut rng)].to_string()
}

fn get_random_user_agent() -> String {
    let sites = USER_AGENT_LIST.split('\n').collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, sites.len() - 1);

    sites[range.sample(&mut rng)].to_string()
}