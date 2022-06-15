use std::env;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use log::debug;
use crate::leetcode::cache::DB_KEYS;
use crate::leetcode::net::problems_all::ProblemsAll;
use crate::leetcode::error::Result;
use crate::leetcode::lang::Language;
use crate::leetcode::net::question_data::QuestionData;


pub struct PickPlugin {
    problems_all: Option<ProblemsAll>,
    question_data: Option<QuestionData>,
    question_id: i32,
    question_title_slug: String,
}

impl PickPlugin {
    pub fn new() -> PickPlugin {
        PickPlugin {
            problems_all: None,
            question_data: None,
            question_id: 0,
            question_title_slug: "".to_string(),
        }
    }

    pub async fn fetch_problems_all(&mut self) -> Result<()> {
        self.problems_all = Some(ProblemsAll::fetch().await?);
        Ok(())
    }

    pub async fn fetch_question_data(&mut self, question_id: i32) -> Result<()> {
        self.question_id = question_id;
        self.question_title_slug = self.problems_all.as_ref()
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
        self.question_data = Some(QuestionData::fetch(&self.question_title_slug).await?);
        debug!("QuestionData: {}", serde_json::to_string_pretty(
                self.question_data.as_ref().unwrap()
            ).unwrap());
        Ok(())
    }

    pub async fn save_to_file(&mut self, language: Option<String>) -> Result<()> {
        let question_data = self.question_data.as_ref().unwrap();

        let language = self.parse_language(language).await.unwrap_or(Language::C);
        crate::leetcode::cache::set(DB_KEYS.language.to_string(), language.name.to_string()).await?;

        let mut filename = env::current_dir()?;
        filename.push(format!("{}-{}", self.question_id, self.question_title_slug));
        filename.set_extension(language.extension);

        let comment_content = if !question_data.translated_content.is_empty() {
            wrap_content_with_comment(
                &question_data.translated_content,
                language.single_line_comment,
            )
        } else {
            wrap_content_with_comment(
                &question_data.content,
                language.single_line_comment,
            )
        };
        debug!("comment_content: {}", comment_content);

        let code_content = question_data.code_definition
            .iter()
            .find(|cd| {
                cd.value == language.name
            })
            .expect("no code definition for this language")
            .default_code
            .as_str();
        debug!("code content: {}", code_content);

        let mut file = File::create(filename)
            .expect("create file failed");

        file.write(comment_content.as_bytes())?;
        file.write("\n\n".as_bytes())?;
        file.write(code_content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    async fn parse_language(&self, language: Option<String>) -> Result<Language<'static>> {
        return match language {
            None => {
                // 从Cache中读取
                if let Some(s) = crate::leetcode::cache::get(DB_KEYS.language).await? {
                    Ok(Language::from_str(s.as_str())?)
                } else {
                    Ok(Language::C)
                }
            }
            Some(s) => {
                Ok(Language::from_str(&s)?)
            }
        };
    }
}

fn wrap_content_with_comment(content: &str, comment: &str) -> String {
    let content = html2text::from_read(content.as_bytes(), 80);
    content.lines()
        .map(|line| {
            let mut new_line = comment.to_string();
            new_line.push(' ');
            new_line.push_str(line);
            new_line
        })
        .collect::<Vec<String>>()
        .join("\n")
}
