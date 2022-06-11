use log::debug;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::leetcode::config::CONST_CONFIG;
use crate::leetcode::db::DB_KEYS;

use crate::leetcode::error::{LeetcodeError, Result};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionData {
    pub content: String,

    #[serde(with = "serde_with::json::nested")]
    pub stats: Stats,

    pub likes: i32,
    pub dislikes: i32,

    #[serde(with = "serde_with::json::nested")]
    pub code_definition: Vec<CodeDefinition>,

    pub sample_test_case: String,
    pub enable_run_code: bool,

    #[serde(with = "serde_with::json::nested")]
    pub meta_data: MetaData,

    pub translated_content: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_accepted: String,
    pub total_submission: String,
    pub total_accepted_raw: i32,
    pub total_submission_raw: i32,
    pub ac_rate: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    pub default_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub params: Vec<MetaDataParam>,

    #[serde(rename = "return")]
    pub ret: MetaDataReturn,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaDataParam {
    pub name: String,

    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaDataReturn {
    #[serde(rename = "type")]
    pub typ: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    question: QuestionData,
}

impl QuestionData {
    pub async fn fetch(question_title_slug: &str) -> Result<QuestionData> {
        let cookie = crate::leetcode::db::get(DB_KEYS.cookie).await?.unwrap_or("".to_string());
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
        let client = Client::builder()
            .default_headers(headers)
            .build()?;
        let j = json!({
            "query": r#"
                query getQuestionData($titleSlug: String!) {
                   question(titleSlug: $titleSlug) {
                     content
                     stats
                     likes
                     dislikes
                     codeDefinition
                     sampleTestCase
                     enableRunCode
                     metaData
                     translatedContent
                   }
                }
            "#,
            "variables": json!({
                "titleSlug": question_title_slug,
            }),
            "operationName": "getQuestionData"
        });
        let res = client.post(CONST_CONFIG.url.leetcode.graphql)
            .json(&j)
            .send()
            .await?
            .json::<Response>()
            .await
            .map_err(LeetcodeError::Reqwest)?;
        debug!("Response: {}", serde_json::to_string_pretty(&res).unwrap());
        Ok(res.data.question)
    }
}
