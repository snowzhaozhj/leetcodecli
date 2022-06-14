extern crate core;

mod leetcode;

use leetcode::error::Result;

#[tokio::main]
async fn main() -> Result<()>{
    env_logger::init();
    leetcode::cli::process().await?;
    Ok(())
}
