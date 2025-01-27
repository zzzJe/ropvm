use crate::command::{Cli, Commands};

mod add;
mod apply;
mod config;
mod list;
mod load;
mod remove;
mod search;
mod util;

pub(super) use list::SortBy as ListSortBy;

pub async fn command_handler(cli: Cli) {
    let cmd = cli.command;
    match cmd {
        Commands::Add { versions } => add::handler(versions).await,
        Commands::Remove { patterns } => remove::handler(patterns).await,
        Commands::Apply { version } => apply::handler(version).await,
        Commands::Config {
            minecraft_dir,
            java_path,
            repo_dir,
            test,
        } => config::handler(minecraft_dir, java_path, repo_dir, test).await,
        Commands::List {
            pattern,
            load_order,
            time,
            by,
        } => list::handler(pattern, load_order, time, by).await,
        Commands::Load => load::handler().await,
        Commands::Search { version } => search::handler(version).await,
    }
}
