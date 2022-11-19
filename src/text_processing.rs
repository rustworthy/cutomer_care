#![allow(dead_code)]

use error_handling::ServiceError;
use serde::Deserialize;
use std::env;
use tracing::{event, Level};

#[derive(Debug, Clone, Deserialize)]
pub struct BadWordsServiceOkResponse {
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
    #[serde(rename = "replacedLen")]
    pub replaced_len: i64,
}

#[derive(Deserialize)]
struct BadWordsServiceErrorResponse {
    message: String,
}

pub async fn filter_out_bad_words(text: String) -> Result<String, ServiceError> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("APIKEY", env::var("BAD_WORDS_SERVICE_API_KEY").unwrap_or_default())
        .body(text)
        .send()
        .await
        .map_err(|e| {
            event!(Level::ERROR, "Error fetching data from Bad Words serviceL {}", e);
            ServiceError::ExternalApiError
        })?;

    if !res.status().is_success() {
        let status = res.status().as_u16();
        let msg = match res.json::<BadWordsServiceErrorResponse>().await {
            Ok(resp) => resp.message,
            Err(_) => return Err(ServiceError::ExternalApiError),
        };
        event!(
            Level::ERROR,
            "Error occurred when calling external API (BadWords Service). Response status {}. Message: {}",
            status,
            msg
        );
        return Err(ServiceError::ExternalApiError);
    }

    match res.json::<BadWordsServiceOkResponse>().await {
        Ok(resp) => Ok(resp.censored_content),
        Err(e) => {
            event!(
                Level::ERROR,
                "Error occurred when calling external API (BadWords Service): {}",
                e
            );
            Err(ServiceError::ExternalApiError)
        }
    }
}
