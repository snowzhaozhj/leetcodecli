use crate::leetcode::error::{LeetcodeError, Result};
use crate::leetcode::lang::Language;
use crate::leetcode::net::submit::{submit, SubmitArgs};
use std::fs::File;
use std::io::Read;
use anyhow::anyhow;
use log::debug;
use crate::leetcode::net::judge::JudgeResult;

pub struct SubmitPlugin {
    submission_id: String,
    language: Option<Language<'static>>,
}

impl SubmitPlugin {
    pub fn new() -> SubmitPlugin {
        SubmitPlugin {
            submission_id: String::new(),
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

    pub async fn submit_code(&mut self, filename: &str, test_data: Option<String>) -> Result<()> {
        if let Ok((id, slug, ext)) = sscanf::scanf!(filename, "{usize}-{str}.{str}") {
            self.language = Some(Language::from_extension(&ext)
                .expect("extension not support"));

            let typed_code = self
                .read_code_from_file(filename)
                .expect("read code failed");
            debug!("typed_code: {}", typed_code);

            let submit_args = match test_data {
                None => {
                    SubmitArgs::new(
                        self.language.as_ref().unwrap().name.to_string(),
                        slug.to_string(),
                        id.to_string(),
                        typed_code,
                    )
                }
                Some(mut data_input) => {
                    data_input = data_input.replace("\\n", "\n");
                    SubmitArgs::new_test(
                        slug.to_string(),
                        data_input,
                        self.language.as_ref().unwrap().name.to_string(),
                        id.to_string(),
                        typed_code,
                    )
                }
            };
            debug!("submit args: {}", serde_json::to_string_pretty(&submit_args).unwrap());

            self.submission_id = submit(submit_args).await.expect("submit failed");
            debug!("submission_id: {}", self.submission_id);
            Ok(())
        } else {
            Err(LeetcodeError::Any(anyhow!("file invalid")))
        }
    }

    pub async fn show_judge_result(&self) -> Result<()> {
        JudgeResult::get(self.submission_id.as_str())
            .await
            .expect("get judge result failed")
            .pretty_print();
        Ok(())
    }
}
