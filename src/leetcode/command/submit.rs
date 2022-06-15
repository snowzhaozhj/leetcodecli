use crate::leetcode::error::{LeetcodeError, Result};
use crate::leetcode::lang::Language;
use crate::leetcode::net::submit::{submit, SubmitArgs};
use std::fs::File;
use std::io::Read;
use anyhow::anyhow;
use log::debug;
use crate::leetcode::net::judge::JudgeResult;

pub struct SubmitPlugin {
    submission_id: usize,
    language: Option<Language<'static>>,
}

impl SubmitPlugin {
    pub fn new() -> SubmitPlugin {
        SubmitPlugin {
            submission_id: 0,
            language: None,
        }
    }

    fn read_code_from_file(&self, filename: &str) -> Result<String> {
        let mut content = String::new();
        File::open(filename)
            .expect("fail to open file")
            .read_to_string(&mut content)?;
        content = content.lines()
            .filter(|l| {
                !l.starts_with(self.language.as_ref().unwrap().single_line_comment)
            })
            // .map(|l| {
            //     l.to_string()
            // })
            .collect::<Vec<&str>>()
            .join("\n");
        Ok(content)
    }

    pub async fn submit_code(&mut self, filename: &str) -> Result<()> {
        if let Ok((id, slug, ext)) = sscanf::scanf!(filename, "{usize}-{str}.{str}") {
            self.language = Some(Language::from_extension(&ext)
                .expect("extension not support"));
            let typed_code = self
                .read_code_from_file(filename)
                .expect("read code failed");
            debug!("typed_code: {}", typed_code);
            self.submission_id = submit(SubmitArgs::new(
                self.language.as_ref().unwrap().name.to_string(),
                slug.to_string(),
                id.to_string(),
                typed_code,
            )).await.expect("submit failed").submission_id;
            debug!("submission_id: {}", self.submission_id);
            Ok(())
        } else {
            Err(LeetcodeError::Any(anyhow!("file invalid")))
        }
    }

    pub async fn show_judge_result(&self) -> Result<()> {
        JudgeResult::get(self.submission_id)
            .await
            .expect("get judge result failed")
            .pretty_print();
        Ok(())
    }
}
