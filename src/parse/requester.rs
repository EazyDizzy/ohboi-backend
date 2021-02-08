use actix_web::web::Bytes;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use reqwest::header::USER_AGENT;
use reqwest::Response;

pub async fn get_data(url: String) -> Result<String, reqwest::Error> {
    let response = get_request(url).await?;
    println!("response headers {:?}", response.headers());

    let text = response.text().await?;

    Ok(text)
}

pub async fn get_bytes(url: String) -> Result<Bytes, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?)
}

pub async fn get_request(url: String) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let req = client
        .get(url.as_str())
        .header(USER_AGENT, get_random_user_agent())
        .header("Accept-Language", "en-gb")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept", "*/*")
        .header("Referer", get_random_referer())
        ;
    // TODO preconnect, zip header
    // TODO proxy
    println!("request headers {:?}", req.try_clone().unwrap().build().unwrap().headers());

    req
        .send()
        .await
}

fn get_random_referer() -> String {
    let referer_list = include_str!("../../cache/referrers");
    let referrers = referer_list.split("\n").collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, referrers.len() - 1);

    referrers[range.sample(&mut rng)].to_string()
}

fn get_random_user_agent() -> String {
    let site_list = include_str!("../../cache/user_agents");
    let sites = site_list.split("\n").collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, sites.len() - 1);

    sites[range.sample(&mut rng)].to_string()
}