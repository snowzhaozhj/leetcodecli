use crate::{
    leetcode::error::Result,
};
use clap::{Subcommand, Args, Parser};
use log::debug;
use crate::leetcode::app::Leetcode;

#[derive(Debug, Parser)]
#[clap(name = "leetcodecli")]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// login or logout
    Auth(Auth),

    /// list problems
    List(List),

    /// pick a problem
    Pick(Pick),

    /// submit your answer
    Submit(Submit),
}

#[derive(Debug, Args)]
struct Auth {
    #[clap(subcommand)]
    command: AuthCommands,
}

#[derive(Debug, Subcommand)]
enum AuthCommands {
    /// user login
    Login {
        /// login mode
        #[clap(default_value = "cookie", possible_values = ["cookie", "git"])]
        mode: String,
    },

    /// user logout
    Logout {
        /// logout mode
        #[clap(default_value = "cookie", possible_values = ["cookie", "git"])]
        mode: String,
    },
}

#[derive(Debug, Args)]
pub struct List {
    /// filter by keyword
    #[clap(short, long)]
    pub keyword: Option<String>,

    /// filter by tag
    #[clap(short, long)]
    pub tag: Option<String>,

    /// filter by difficulty
    #[clap(short, long)]
    pub difficulty: Option<String>,

    /// filter by star status
    #[clap(short, long)]
    pub star: Option<bool>,

    /// filter by lock status
    #[clap(short, long)]
    pub lock: Option<bool>,

    /// filter by finish status
    #[clap(short, long)]
    pub finish: Option<bool>,

    /// order by `problem id`, `title`, `difficulty`
    #[clap(short, long)]
    pub order: Option<String>,
}

#[derive(Debug, Args)]
pub struct Pick {
    /// the id of the question
    pub question_id: i32,

    /// programming language to write answer
    #[clap(short, long)]
    pub language: Option<String>,
}

#[derive(Debug, Args)]
pub struct Submit {
    /// the source code file to submit
    pub filename: String,

    /// test mode, "\n" in test_data will be turn into new line
    #[clap(short, long)]
    pub test_data: Option<String>,
}

pub async fn process() -> Result<()> {
    let cli: Cli = Cli::parse();
    debug!("Cli: {:#?}", cli);
    let mut app = Leetcode::new();
    match cli.command {
        Commands::Auth(auth) => {
            match auth.command {
                AuthCommands::Login { mode } => {
                    app.login(mode).await?;
                }
                AuthCommands::Logout { mode } => {
                    debug!("auth logout mode: {}", mode);
                    app.logout(mode).await?;
                }
            }
        }
        Commands::List(list) => {
            if let Some(k) = list.keyword {
                println!("list keyword: {}", k);
            }
            if let Some(t) = list.tag {
                println!("list tag: {}", t);
            }
            if let Some(s) = list.star {
                println!("list star: {}", s);
            }
            if let Some(d) = list.difficulty {
                println!("list difficulty: {}", d);
            }
            if let Some(l) = list.lock {
                println!("list lock: {}", l);
            }
            if let Some(o) = list.order {
                print!("list order: {}", o);
            }
            app.list_problems().await?;
        }
        Commands::Pick(pick) => {
            app.pick_problem(pick).await?;
        }
        Commands::Submit(submit) => {
            app.sumbit(submit).await?;
        }
    }
    Ok(())
}
