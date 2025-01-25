use clap::Parser;

mod command;
mod db;
mod index;
mod scrape;

#[tokio::main]
async fn main() {
    let cli = command::Cli::parse();
    command::command_handler(cli).await;
}
