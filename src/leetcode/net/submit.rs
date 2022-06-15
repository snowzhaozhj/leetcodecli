use log::debug;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use crate::leetcode::config::CONST_CONFIG;
use crate::leetcode::db::DB_KEYS;

use crate::leetcode::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitArgs {
    pub lang: String,

    #[serde(rename = "questionSlug")]
    pub question_slug: String,

    pub question_id: String,
    pub test_judger: String,
    pub test_mode: bool,
    pub typed_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitResult {
    pub submission_id: usize,
}

impl SubmitArgs {
    pub fn new(lang: String, question_slug: String, question_id: String, typed_code: String) -> SubmitArgs {
        SubmitArgs {
            lang,
            question_slug,
            question_id,
            test_judger: "".to_string(),
            test_mode: false,
            typed_code,
        }
    }
}

pub async fn submit(args: SubmitArgs) -> Result<SubmitResult> {
    let referer_url = CONST_CONFIG.url.leetcode.submissions.replace("$slug", args.question_slug.as_str());
    let post_url = CONST_CONFIG.url.leetcode.submit.replace("$slug", args.question_slug.as_str());
    let cookie = crate::leetcode::db::get(DB_KEYS.cookie).await?.unwrap_or("".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
    headers.insert("referer", HeaderValue::from_str(referer_url.as_str()).unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    let res = client.post(post_url)
        .json(&args)
        .send()
        .await?
        .json::<SubmitResult>()
        .await?;
    debug!("res: {}", serde_json::to_string_pretty(&res).unwrap());
    Ok(res)
}
