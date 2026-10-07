use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rule34 downloader with resume capability", long_about = None)]
pub struct Args {
    #[arg(short, long, required = true, help = "Tags separated by space (e.g., \"tag1 tag2 -bad_tag\")")]
    pub tags: String,

    #[arg(short = 'n', long, default_value_t = 20, help = "Limit of posts to download (default: 20)")]
    pub limit: usize,

    #[arg(short, long, help = "Output directory (default: tags as folder name)")]
    pub output: Option<PathBuf>,

    #[arg(short, long, help = "Path to config file")]
    pub config: Option<PathBuf>,

    #[arg(long, help = "Show what would be downloaded without downloading")]
    pub dry_run: bool,

    #[arg(long, help = "Download without saving to history")]
    pub no_history: bool,

    #[arg(long, help = "Check file hashes for duplicates")]
    pub check_hash: bool,

    #[arg(long, help = "Skip GIF files entirely (not counted in limit)")]
    pub skip_gif: bool,
}
