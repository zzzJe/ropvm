use clap::{Parser, Subcommand};

mod handler;
mod style;
pub use handler::command_handler;

#[derive(Parser)]
#[command(name = "opvm")]
#[command(version = "0.1.0")]
#[command(about = "Optifine version manager", long_about = None)]
#[command(styles = style::CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add Optifine version(s) into local repo
    Add {
        #[arg(help = "\
            Some Minecraft versions, or some specific Optifine versions\n\
            Version can be:\n\
            * Minecraft Version[Index Range]\n  \
              - 1.16.5[1]\n  \
              - 1.16.5[1~]\n  \
              - 1.16.5[~2]\n  \
              - 1.16.5[1~2]\n  \
              - 1.16.5[~]\n  \
              - 1.16.5[1/3~]\n\
            * Minecraft Version\n  \
              - 1.16.5 (= 1.16.5[1])\n  \
              - 1.20.4 (= 1.20.4[1])\n\
            * Optifine Version\n  \
              - 1.16.5_HD_U_G8\
        ")]
        versions: Vec<String>,
    },
    /// Remove Optifine version(s) from local repo
    Remove {
        #[arg(help = "\
            Some local Optifine versions pattern\n\
            Pattern can be:\n\
            * Complete Optifine Version\n\
            * Partial Optifine Version (delete all version that contain this pattern)\n\
            * Minecraft Version (delete all version of that Minecraft version)\n\
            * Any name\n\
            * \"\" (empty string) (meaning delete all files)\n\
        ")]
        patterns: Vec<String>,
    },
    // TODO?: Minecraft Version = Minecraft Version[1]
    /// Apply Optifine by opening setting GUI
    Apply {
        #[arg(help = "\
            A local Minecraft version, or specific Optifine version\n\
            Version can be:\n\
            * Minecraft Version[Index]\n\
            * Pattern (can be Optifine Version)\n\
        ")]
        version: String,
    },
    /// Config on opvm
    Config {
        #[arg(short, long, help = "Minecraft root directory")]
        minecraft_dir: Option<String>,
        #[arg(short, long, help = "Java machine(executable) path")]
        java_path: Option<String>,
        #[arg(short, long, help = "Local repo to store Optifine files")]
        repo_dir: Option<String>,
        #[arg(short, long, help = "Validate config fields correctness")]
        test: bool,
    },
    /// List downloaded Optifine versions
    List {
        #[arg(help = "Empty or a pattern")]
        pattern: Option<String>,
        #[arg(short, long, help = "Force to load version order")]
        load_order: bool,
        #[arg(short, long, help = "Display download time")]
        time: bool,
        #[arg(short, long, help = "Version display order")]
        by: Option<handler::ListSortBy>,
    },
    /// Search for avaliable Optifine versions
    Search {
        #[arg(help = "Empty or a Minecraft version")]
        version: Option<String>,
    },
    /// Load all Optifine files in configured local repo
    Load,
}
