use std::collections::HashMap;
use async_trait::async_trait;
use cookie::{Cookie, CookieJar};
use log::debug;
use crate::leetcode::cache::DB_KEYS;

use crate::leetcode::error::Result;

#[async_trait]
pub trait AuthPlugin {
    fn name(&self) -> String;

    async fn login(&mut self) -> Result<()>;
    async fn logout(&mut self) -> Result<()>;
}

pub(crate) fn get_plugins() -> HashMap<String, Box<dyn AuthPlugin>> {
    let mut auth_plugins: HashMap<String, Box<dyn AuthPlugin>> = HashMap::new();
    let cookie_auth_plugin = Box::new(CookieAuthPlugin::new());
    let git_auth_plugin = Box::new(GitAuthPlugin::new());
    auth_plugins.insert(cookie_auth_plugin.name(), cookie_auth_plugin);
    auth_plugins.insert(git_auth_plugin.name(), git_auth_plugin);
    auth_plugins
}

pub struct CookieAuthPlugin {
    cookie: String,
}

impl CookieAuthPlugin {
    fn new() -> CookieAuthPlugin {
        CookieAuthPlugin {
            cookie: "".to_string(),
        }
    }

    fn init_cookie(&mut self) -> Result<()> {
        let mut session = String::new();
        let mut csrftoken = String::new();
        println!("Enter session:");
        std::io::stdin()
            .read_line(&mut session)
            .expect("fail to read session");
        session = format!(r#"{}"#, session.trim_end());
        debug!("session: {}", session);
        println!("Enter csrftoken:");
        std::io::stdin()
            .read_line(&mut csrftoken)
            .expect("fail to read csrftoken");
        csrftoken = format!(r#"{}"#, csrftoken.trim_end());
        debug!("csrftoken: {}", csrftoken);
        let mut jar = cookie::CookieJar::new();
        jar.add(Cookie::new("LEETCODE_SESSION", session));
        jar.add(Cookie::new("csrftoken", csrftoken));
        self.cookie = cookie_jar_to_string(&jar);
        debug!("cookie: {}", self.cookie);
        Ok(())
    }

    async fn store_cookie(&mut self) -> Result<()> {
        crate::leetcode::cache::set(DB_KEYS.cookie.to_string(), self.cookie.clone()).await?;
        Ok(())
    }

    async fn remove_cookie(&self) -> Result<()> {
        crate::leetcode::cache::remove(DB_KEYS.cookie).await?;
        Ok(())
    }
}

#[async_trait]
impl AuthPlugin for CookieAuthPlugin {
    fn name(&self) -> String {
        "cookie".to_owned()
    }

    async fn login(&mut self) -> Result<()> {
        self.init_cookie()?;
        self.store_cookie().await?;
        Ok(())
    }

    async fn logout(&mut self) -> Result<()> {
        self.remove_cookie().await?;
        Ok(())
    }
}

pub struct GitAuthPlugin {
    cookie: String,
}

impl GitAuthPlugin {
    fn new() -> GitAuthPlugin {
        GitAuthPlugin {
            cookie: "".to_owned()
        }
    }
}

#[async_trait]
impl AuthPlugin for GitAuthPlugin {
    fn name(&self) -> String {
        "git".to_owned()
    }

    async fn login(&mut self) -> Result<()> {
        todo!()
    }

    async fn logout(&mut self) -> Result<()> {
        todo!()
    }
}

fn cookie_jar_to_string(cookie_jar: &CookieJar) -> String {
    cookie_jar.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}
