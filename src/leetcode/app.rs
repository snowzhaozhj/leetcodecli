use std::collections::HashMap;

use crate::leetcode::error::Result;
use crate::leetcode::command::auth::{self, AuthPlugin};
use crate::leetcode::command::list::ListPlugin;

pub struct Leetcode {
    auth_plugins: HashMap<String, Box<dyn AuthPlugin>>,
    list_plugin: ListPlugin,
}

impl Leetcode {
    pub fn new() -> Leetcode {
        let auth_plugins = auth::get_plugins();
        let list_plugin = ListPlugin::new();
        Leetcode {
            auth_plugins,
            list_plugin,
        }
    }

    pub async fn login(&mut self, mode: String) -> Result<()> {
        self.auth_plugins.get_mut(&mode).expect("error mode")
            .login().await?;
        Ok(())
    }

    pub async fn logout(&mut self, mode: String) -> Result<()> {
        self.auth_plugins.get_mut(&mode).expect("invalid mode")
            .logout().await?;
        Ok(())
    }

    pub async fn list_problems(&mut self) -> Result<()> {
        self.list_plugin.fetch_problems_all().await?;
        self.list_plugin.list_problems_all().await;
        Ok(())
    }
}
