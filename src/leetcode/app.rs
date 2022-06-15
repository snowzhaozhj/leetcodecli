use std::collections::HashMap;
use crate::leetcode::cli::{Pick, Submit};

use crate::leetcode::error::Result;
use crate::leetcode::command::auth::{self, AuthPlugin};
use crate::leetcode::command::list::ListPlugin;
use crate::leetcode::command::pick::PickPlugin;
use crate::leetcode::command::submit::SubmitPlugin;

pub struct Leetcode {
    auth_plugins: HashMap<String, Box<dyn AuthPlugin>>,
    list_plugin: ListPlugin,
    pick_plugin: PickPlugin,
    submit_plugin: SubmitPlugin,
}

impl Leetcode {
    pub fn new() -> Leetcode {
        let auth_plugins = auth::get_plugins();
        let list_plugin = ListPlugin::new();
        let pick_plugin = PickPlugin::new();
        let submit_plugin = SubmitPlugin::new();
        Leetcode {
            auth_plugins,
            list_plugin,
            pick_plugin,
            submit_plugin,
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

    pub async fn pick_problem(&mut self, pick: Pick) -> Result<()> {
        self.pick_plugin.fetch_problems_all().await?;
        self.pick_plugin.fetch_question_data(pick.question_id).await?;
        self.pick_plugin.save_to_file(pick.language).await?;
        Ok(())
    }

    pub async fn sumbit(&mut self, submit: Submit) -> Result<()> {
        self.submit_plugin.submit_code(submit.filename.as_str()).await?;
        self.submit_plugin.show_judge_result().await?;
        Ok(())
    }
}
