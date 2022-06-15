use ansi_term::Color::{Green, Red};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::leetcode::config::CONST_CONFIG;
use crate::leetcode::db::DB_KEYS;
use crate::leetcode::error::Result;
use crate::leetcode::term::icon::Icon;

#[derive(Debug, Serialize, Deserialize)]
pub struct JudgeResult {
    pub status_code: i32,
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: usize,
    pub question_id: String,
    pub elapsed_time: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_result: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_testcase: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<String>,

    pub task_finish_time: usize,
    pub task_name: String,
    pub finished: bool,
    pub status_msg: String,
    pub state: String,
    pub fast_submit: bool,
    pub total_correct: Option<i32>,
    pub total_testcases: Option<i32>,
    pub submission_id: String,
    pub runtime_percentile: Option<f64>,
    pub status_memory: String,
    pub memory_percentile: Option<f64>,
    pub pretty_lang: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_runtime_error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_compile_error: Option<String>,
}

impl JudgeResult {
    pub async fn get(submission_id: usize) -> Result<JudgeResult> {
        let get_url = CONST_CONFIG
            .url
            .leetcode
            .veriry
            .replace("$id", submission_id.to_string().as_str());
        let cookie = crate::leetcode::db::get(DB_KEYS.cookie)
            .await?
            .unwrap_or("".to_string());

        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(cookie.as_str()).unwrap());
        let client = Client::builder().default_headers(headers).build()?;

        let result = loop {
            let res = client
                .get(get_url.as_str())
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;
            debug!("res: {}", serde_json::to_string_pretty(&res).unwrap());
            if res["state"] == "SUCCESS" {
                break serde_json::from_value(res)?;
            }
            std::thread::sleep(Duration::from_millis(200));
        };
        Ok(result)
    }

    pub fn pretty_print(&self) {
        debug!(
            "judge_result: {}",
            serde_json::to_string_pretty(&self).unwrap()
        );
        match self.status_code {
            10 => {
                // Accepted
                let content = format!(
                    r#"
{} {}
{}/{} cases passed ({})
Your runtime beats {}% of {} submissions
Your memory usage beats {}% of {} submissions ({})"#,
                    Icon::Yes.to_string(),
                    self.status_msg,
                    self.total_correct.unwrap(),
                    self.total_testcases.unwrap(),
                    self.status_runtime,
                    self.runtime_percentile.expect("missing runtime percentile field"),
                    self.lang,
                    self.memory_percentile.expect("missing runtime percentile field"),
                    self.lang,
                    self.status_memory
                );
                println!("{}", Green.paint(content));
            }
            15 => {
                // Runtime error
                let content = format!(
                    "{} {}\n{}",
                    Icon::No.to_string(),
                    self.status_msg,
                    self.full_runtime_error.as_ref().expect("missing full runtime error field"),
                );
                println!("{}", Red.paint(content));
            }
            20 => {
                // Compile error
                let content = format!(
                    "{} {}\n{}",
                    Icon::No.to_string(),
                    self.status_msg,
                    self.full_compile_error.as_ref().expect("missing full compile error field")
                );
                println!("{}", Red.paint(content));
            }
            _ => {
                // Wrong answer | TLE
                let content = format!(
                    r#"
{} {}
{}/{} cases passed ({})
Failed Test:
testcase:
{}

code_output:
{}

expectd_output:
{}"#,
                    Icon::No.to_string(),
                    self.status_msg,
                    self.total_correct.unwrap(),
                    self.total_testcases.unwrap(),
                    self.status_runtime,
                    self.last_testcase.as_ref().expect("missing last testcase field"),
                    self.code_output.as_ref().expect("missing code output field"),
                    self.expected_output.as_ref().expect("missing code output field"),
                );
                println!("{}", Red.paint(content));
            }
        }
    }
}
