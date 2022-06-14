use dirs::home_dir;
use tokio::sync::Mutex;
use db::database::DataBase;
use lazy_static::lazy_static;

use crate::leetcode::error::Result;

lazy_static! {
    static ref DB: Mutex<DataBase> = {
        let mut db_path = home_dir().expect("");
        db_path.push(".leetcode");
        db_path.push("db");
        let db = DataBase::open(db_path).expect("fail to open db");
        Mutex::new(db)
    };
}

pub(crate) async fn set(key: String, value: String) -> Result<()> {
    DB.lock().await.set(key, value)?;
    Ok(())
}

pub(crate) async fn get(key: &str) -> Result<Option<String>> {
    let key = DB.lock().await.get(key)?;
    Ok(key)
}

pub(crate) async fn remove(key: &str) -> Result<()> {
    DB.lock().await.remove(key)?;
    Ok(())
}

pub struct DBKeys<'a> {
    pub cookie: &'a str,
    pub problems_all: &'a str,
    pub language: &'a str,
}

pub const DB_KEYS: DBKeys<'static> = DBKeys {
    cookie: "Cookie",
    problems_all: "ProblemsAll",
    language: "Language",
};

