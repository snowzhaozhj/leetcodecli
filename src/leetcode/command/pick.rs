use log::debug;
use crate::leetcode::net::problems_all::ProblemsAll;
use crate::leetcode::error::Result;
use crate::leetcode::net::question_data::QuestionData;

pub struct PickPlugin {
    problems_all: Option<ProblemsAll>,
    question_data: Option<QuestionData>,
}

impl PickPlugin {
    pub fn new() -> PickPlugin {
        PickPlugin {
            problems_all: None,
            question_data: None,
        }
    }

    pub async fn fetch_problems_all(&mut self) -> Result<()> {
        self.problems_all = Some(ProblemsAll::fetch().await?);
        Ok(())
    }

    pub async fn fetch_question_data(&mut self, question_id: i32) -> Result<()> {
        let question_title_slug = self.problems_all.as_ref()
            .expect("fail to fetch problems")
            .stat_status_pairs
            .iter()
            .find(|ss| {
                ss.stat.question_id == question_id
            })
            .expect("question id invalid")
            .stat
            .question_title_slug
            .clone();
        self.question_data = Some(QuestionData::fetch(&question_title_slug).await?);
        debug!("QuestionData: {}", serde_json::to_string_pretty(
                self.question_data.as_ref().unwrap()
            ).unwrap());
        Ok(())
    }
}
