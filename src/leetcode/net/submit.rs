use log::debug;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use crate::leetcode::config::CONST_CONFIG;
use crate::leetcode::db::DB_KEYS;

use crate::leetcode::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitArgs {
    #[serde(rename = "questionSlug")]
    pub question_slug: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_input: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge_type: Option<String>,

    pub lang: String,
    pub question_id: String,
    pub test_judger: String,
    pub test_mode: bool,
    pub typed_code: String,
}

impl SubmitArgs {
    pub fn new(lang: String, question_slug: String, question_id: String, typed_code: String) -> SubmitArgs {
        SubmitArgs {
            question_slug,
            data_input: None,
            judge_type: None,
            lang,
            question_id,
            test_judger: "".to_string(),
            test_mode: false,
            typed_code,
        }
    }

    pub fn new_test(question_slug: String,
                    data_input: String,
                    lang: String,
                    question_id: String,
                    typed_code: String) -> SubmitArgs {
        SubmitArgs {
            question_slug,
            data_input: Some(data_input),
            judge_type: Some("small".to_string()),
            lang,
            question_id,
            test_judger: "".to_string(),
            test_mode: false,
            typed_code,
        }
    }
}

pub async fn submit(args: SubmitArgs) -> Result<String> {
    let referer_url = CONST_CONFIG.url.leetcode.submissions.replace("$slug", args.question_slug.as_str());
    let post_url = if args.data_input.is_none() {
        // 正常提交
        CONST_CONFIG.url.leetcode.submit.replace("$slug", args.question_slug.as_str())
    } else {
        // 测试模式
        CONST_CONFIG.url.leetcode.test.replace("$slug", args.question_slug.as_str())
    };
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
        .json::<serde_json::Value>()
        .await?;
    debug!("res: {}", serde_json::to_string_pretty(&res).unwrap());
    let res = if args.data_input.is_none() {
        // 正常提交
        res["submission_id"].to_string()
    } else {
        // 测试模式
        res["interpret_id"].as_str().unwrap().to_string()
    };
    Ok(res)
}
