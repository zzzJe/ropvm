use clap::{Parser, Subcommand};

mod handler;
mod style;
pub use handler::command_handler;

#[derive(Parser)]
#[command(bin_name = env!("CARGO_PKG_NAME"))]
#[command(version, about, long_about = None)]
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
            Some local Optifine version patterns\n\
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
            A local Minecraft version with index, or a pattern\n\
            Minecraft version with index can be:\n\
            * 1.20.4[] = (1.20.4[1])\n\
            * 1.8.9[5]\n\
            Pattern can be:\n\
            * Any name\n\
            * Custom name\n\
            * An Optifine version\n\
        ")]
        version: String,
    },
    /// Config on opvm
    Config {
        #[arg(short, long, help = "Minecraft root directory")]
        minecraft_dir: Option<String>,
        #[arg(short, long, help = "Java machine(executable) path")]
        java_path: Option<String>,
        #[arg(short, long, help = "File folder to store Optifine files")]
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
