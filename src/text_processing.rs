use error_handling::ServiceError;
use serde::Deserialize;
use std::env;
use tracing::{event, Level};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsServiceResponse {
    pub content: String,
    pub bad_words_total: i64,
    pub bad_words_list: Vec<BadWord>,
    pub censored_content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BadWord {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    pub start: i64,
    pub end: i64,
    #[serde(rename = "camelCase")]
    pub replaced_len: i64,
}

pub async fn _filter_out_bad_words(text: String) -> Result<String, ServiceError> {
    let client = reqwest::Client::new();
    let _res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header(
            "APIKEY",
            env::var("BAD_WORDS_SERVICE_API_KEY").unwrap_or_default(),
        )
        .body(text)
        .send()
        .await
        .map_err(|e| {
            event!(
                Level::ERROR,
                "Error fetching data from Bad Words serviceL {}",
                e
            );
            ServiceError::ExternalApiError
        })?;

    Ok("".to_string())
}
