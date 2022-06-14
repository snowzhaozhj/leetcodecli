use std::str::FromStr;
use anyhow::anyhow;
use crate::leetcode::error::LeetcodeError;

#[derive(Debug, Clone)]
pub struct Language<'a> {
    pub name: &'a str,
    pub extension: &'a str,
    pub single_line_comment: &'a str,
    // 不考虑多行注释
}

impl<'a> Language<'a> {
    pub const C: Language<'static> = Language {
        name: "c",
        extension: "c",
        single_line_comment: "//",
    };

    pub const CPP: Language<'static> = Language {
        name: "cpp",
        extension: "cpp",
        single_line_comment: "//",
    };

    pub const GO: Language<'static> = Language {
        name: "go",
        extension: "go",
        single_line_comment: "//",
    };

    pub const JAVA: Language<'static> = Language {
        name: "java",
        extension: "java",
        single_line_comment: "//",
    };

    pub const PYTHON: Language<'static> = Language {
        name: "python",
        extension: "py",
        single_line_comment: "#",
    };

    pub const RUST: Language<'static> = Language {
        name: "rust",
        extension: "rs",
        single_line_comment: "//",
    };
}

impl<'a> FromStr for Language<'a> {
    type Err = LeetcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::CPP),
            "go" => Ok(Language::GO),
            "java" => Ok(Language::JAVA),
            "python" => Ok(Language::PYTHON),
            "rust" => Ok(Language::RUST),
            _ => Err(LeetcodeError::Any(anyhow!("Language not supported!")))
        }
    }
}
