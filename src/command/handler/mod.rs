use crate::command::{Cli, Commands};

mod add;
mod apply;
mod config;
mod list;
mod load;
mod remove;
mod search;
mod util;

pub use list::SortBy as ListSortBy;

#[allow(unused)]
pub async fn command_handler(cli: Cli) {
    let cmd = cli.command;
    match cmd {
        Commands::Add { versions } => add::handler(versions).await,
        Commands::Remove { versions } => {}
        Commands::Apply { version } => {}
        Commands::Config {
            minecraft_dir,
            java_path,
            repo_dir,
            test,
        } => config::handler(minecraft_dir, java_path, repo_dir, test).await,
        Commands::List {
            version,
            load_order,
            time,
            by,
        } => list::handler(version, load_order, time, by).await,
        Commands::Load => load::handler().await,
        Commands::Search { version } => search::handler(version).await,
    }
}
