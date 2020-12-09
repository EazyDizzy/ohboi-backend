use crate::parse;

pub async fn parse_google() -> Result<String, reqwest::Error> {
    parse::requester::get_data("http://google.com").await
}