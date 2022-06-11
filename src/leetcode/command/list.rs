use crate::leetcode::net::problems_all::ProblemsAll;
use crate::leetcode::error::Result;

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
        self.problems_all = Some(ProblemsAll::fetch().await?);
        Ok(())
    }

    pub async fn list_problems_all(&self) {
        self.problems_all.as_ref()
            .expect("fail to fetch problems")
            .stat_status_pairs
            .iter()
            .for_each(|s| {
                s.pretty_print();
            });
    }
}
