//! 访问 https://leetcode.cn/api/problems/all/ 返回的结构体

use ansi_term::Color::{Green, Red, Yellow};
use log::debug;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use crate::leetcode::config::CONST_CONFIG;
use crate::leetcode::cache::DB_KEYS;
use crate::leetcode::term::icon::Icon;
use crate::leetcode::error::{LeetcodeError, Result};

#[derive(Serialize, Deserialize)]
pub struct ProblemsAll {
    pub user_name: String,
    pub num_solved: i32,
    pub num_total: i32,
    pub ac_easy: i32,
    pub ac_medium: i32,
    pub ac_hard: i32,
    pub stat_status_pairs: Vec<StatStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct StatStatus {
    pub stat: Stat,
    pub status: Option<String>,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: i32,
    pub progress: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub question_id: i32,

    #[serde(rename = "question__title")]
    pub question_title: String,

    #[serde(rename = "question__title_slug")]
    pub question_title_slug: String,

    #[serde(rename = "question__hide")]
    pub question_hide: bool,

    pub total_acs: i32,
    pub total_submitted: i32,
    pub total_column_articles: i32,
    pub frontend_question_id: String,
    pub is_new_question: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Difficulty {
    pub level: i32,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self.level {
            1 => Green.paint("easy").to_string(),
            2 => Yellow.paint("medium").to_string(),
            3 => Red.paint("hard").to_string(),
            _ => panic!("unexpected level")
        }
    }
}

fn to_width(s: &str, width: usize) -> String {
    let cur_width = unicode_width::UnicodeWidthStr::width(s);
    if cur_width > width {
        let mut s = s[..width-3].to_string();
        s.push_str("...");
        return s;
    }
    let mut s = s.to_string();
    s.push_str(" ".repeat(width - cur_width).as_str());
    s
}

impl StatStatus {
    pub fn pretty_print(&self) {
        let starred_icon = if self.is_favor {
            Yellow.paint(Icon::Star.to_string()).to_string()
        } else {
            Icon::Empty.to_string()
        };

        let locked_icon = if self.paid_only {
            Red.paint(Icon::Lock.to_string()).to_string()
        } else {
            Icon::Empty.to_string()
        };

        let accepted_icon = if self.status.is_some() {
            Green.paint(Icon::Yes.to_string()).to_string()
        } else {
            Icon::Empty.to_string()
        };

        println!(
            "{} {:2} {} [{:^8}] {} {:6}",
            starred_icon,
            locked_icon,
            accepted_icon,
            self.stat.question_id,
            to_width(self.stat.question_title.as_str(), 75),
            self.difficulty.to_string(),
        )
    }
}

impl ProblemsAll {
    pub async fn fetch() -> Result<ProblemsAll> {
        let mut problems_all: ProblemsAll;
        if let Some(val) = crate::leetcode::cache::get(DB_KEYS.problems_all).await? {
            problems_all = serde_json::from_str(&val)?;
        } else {
            let cookie = crate::leetcode::cache::get(DB_KEYS.cookie).await?.unwrap_or("".to_string());
            let mut headers = HeaderMap::new();
            headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
            let client = Client::builder()
                .default_headers(headers)
                .build()?;
            problems_all = client.get(CONST_CONFIG.url.leetcode.problems_all)
                .send()
                .await?
                .json::<ProblemsAll>()
                .await
                .map_err(LeetcodeError::Reqwest)?;
            problems_all.stat_status_pairs
                .sort_by_key(|ss| {
                    ss.stat.question_id
                });
            crate::leetcode::cache::set(
                DB_KEYS.problems_all.to_string(),
                serde_json::to_string(&problems_all).unwrap())
                .await?;
        }
        debug!("ProblemsAll: {}", serde_json::to_string_pretty(&problems_all).unwrap());
        Ok(problems_all)
    }
}
