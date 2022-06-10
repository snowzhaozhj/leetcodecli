use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::leetcode::db::{DB_KEYS};
use crate::leetcode::error::{LeetcodeError, Result};
use crate::leetcode::structs::problems_all::ProblemsAll;

pub struct ListPlugin {
    problems_all: Option<ProblemsAll>,
}

impl ListPlugin {
    pub fn new() -> ListPlugin {
        ListPlugin {
            problems_all: None,
        }
    }

    pub async fn fetch_problems_all(&mut self) -> Result<()> {
        if let Some(val) = crate::leetcode::db::get(DB_KEYS.problems_all).await? {
            self.problems_all = Some(serde_json::from_str(&val)?);
        } else {
            let cookie = crate::leetcode::db::get(DB_KEYS.cookie).await?.unwrap_or("".to_string());
            let mut headers = HeaderMap::new();
            headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
            let client = Client::builder()
                .default_headers(headers)
                .build()?;
            self.problems_all = Some(client.get("https://leetcode.cn/api/problems/all")
                .send()
                .await?
                .json::<ProblemsAll>()
                .await
                .map_err(LeetcodeError::Reqwest)?
            );
            self.problems_all.as_mut()
                .unwrap()
                .stat_status_pairs
                .sort_by_key(|ss| {
                    ss.stat.question_id
                });
            crate::leetcode::db::set(DB_KEYS.problems_all.to_string(),
                                     serde_json::to_string(self.problems_all.as_ref().unwrap())?)
                .await?;
        }
        Ok(())
    }

    pub async fn list_problems_all(&self) {
        self.problems_all.as_ref()
            .expect("problems_all is none")
            .stat_status_pairs
            .iter()
            .for_each(|s| {
                s.pretty_print();
            });
    }
}
