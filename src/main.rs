mod api;
mod cli;
mod config;
mod downloader;
mod history;

use clap::Parser;
use cli::Args;
use config::load_config;
use downloader::download_posts;
use history::load_history;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = load_config(args.config)?;

    let (formatted_tags, api_tags) = match api::prepare_tags(&args.tags, &config) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let target_dir = args.output.unwrap_or_else(|| PathBuf::from(&formatted_tags));
    tokio::fs::create_dir_all(&target_dir).await?;

    let history = load_history(&target_dir);

    let client = reqwest::Client::new();
    let posts = api::fetch_posts(
        &client,
        &config,
        &api_tags,
        args.limit,
        args.skip_gif,
        &history,
    )
    .await?;

    if posts.is_empty() {
        println!("No posts found for tags: {}", args.tags);
        return Ok(());
    }

    if args.dry_run {
        for post in &posts {
            println!("[DRY-RUN] Would download: {}", post.file_url);
        }
        return Ok(());
    }

    let parallel_downloads = config.parallel_downloads.unwrap_or(4);

    download_posts(
        client,
        posts,
        target_dir,
        formatted_tags,
        history,
        parallel_downloads,
        args.check_hash,
        args.no_history,
    )
    .await?;

    Ok(())
}
