use crate::api::Post;
use crate::history::{save_history, History};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;

pub const ALLOWED_EXTS: &[&str] = &["jpg", "jpeg", "png", "gif", "webm", "mp4"];

pub fn count_existing_media(dir: &Path) -> usize {
    if !dir.exists() {
        return 0;
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|os_str| os_str.to_str())
                    .map(|ext| ALLOWED_EXTS.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .count()
    } else {
        0
    }
}

pub async fn download_posts(
    client: reqwest::Client,
    posts: Vec<Post>,
    target_dir: PathBuf,
    formatted_tags: String,
    history: History,
    parallel_downloads: usize,
    check_hash: bool,
    no_history: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_index_offset = count_existing_media(&target_dir);
    let history = Arc::new(Mutex::new(history));
    let file_index = Arc::new(AtomicUsize::new(file_index_offset));
    let formatted_tags = Arc::new(formatted_tags);
    let target_dir = Arc::new(target_dir);
    let client = Arc::new(client);
    let multi_progress = Arc::new(MultiProgress::new());

    let mut tasks = tokio::task::JoinSet::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(parallel_downloads.max(1)));

    for post in posts {
        let history = Arc::clone(&history);
        let file_index = Arc::clone(&file_index);
        let formatted_tags = Arc::clone(&formatted_tags);
        let target_dir = Arc::clone(&target_dir);
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let multi_progress = Arc::clone(&multi_progress);

        tasks.spawn(async move {
            let _permit = match semaphore.acquire().await {
                Ok(p) => p,
                Err(_) => return,
            };

            let ext = Path::new(&post.file_url)
                .extension()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("bin")
                .to_lowercase();

            let tmp_filename = format!(".tmp_{}_{}.part", post.id, std::process::id());
            let tmp_path = target_dir.join(&tmp_filename);

            let mut file_res = match client.get(&post.file_url).send().await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to start download for post {}: {}", post.id, e);
                    return;
                }
            };

            if !file_res.status().is_success() {
                eprintln!("Failed to download post {}: HTTP {}", post.id, file_res.status());
                return;
            }

            let total_size = file_res
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|ct| ct.to_str().ok())
                .and_then(|ct| ct.parse::<u64>().ok())
                .unwrap_or(0);

            let pb = multi_progress.add(ProgressBar::new(total_size));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{elapsed_precise}] [{bar:20.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            pb.set_message(format!("↓ post {}.{}", post.id, ext));

            // Use tokio::fs::File and AsyncWriteExt to avoid blocking Tokio worker threads
            let mut async_file = match tokio::fs::File::create(&tmp_path).await {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to create temporary file {}: {}", tmp_path.display(), e);
                    return;
                }
            };

            let mut hasher = Sha256::new();
            let mut download_succeeded = true;

            while let Ok(Some(chunk)) = file_res.chunk().await {
                if let Err(e) = async_file.write_all(&chunk).await {
                    eprintln!("Error writing chunk for post {}: {}", post.id, e);
                    download_succeeded = false;
                    break;
                }
                hasher.update(&chunk);
                pb.inc(chunk.len() as u64);
            }

            if !download_succeeded {
                pb.finish_and_clear();
                let _ = tokio::fs::remove_file(&tmp_path).await;
                return;
            }

            if let Err(e) = async_file.flush().await {
                eprintln!("Error flushing file for post {}: {}", post.id, e);
                pb.finish_and_clear();
                let _ = tokio::fs::remove_file(&tmp_path).await;
                return;
            }
            drop(async_file);

            let hash = hex::encode(hasher.finalize());

            // Check duplicate hash before reserving final filename or index
            if check_hash {
                let is_duplicate = {
                    let hist = history.lock().unwrap();
                    hist.file_hashes.contains(&hash)
                };
                if is_duplicate {
                    pb.finish_and_clear();
                    println!("Skipping duplicate post {} (hash match: {})", post.id, hash);
                    let _ = tokio::fs::remove_file(&tmp_path).await;
                    return;
                }
            }

            // Reserve filename only after successful download and verification.
            // Using AtomicUsize without rollback guarantees that errors/skips never cause overwrites.
            let (final_filename, final_filepath) = loop {
                let idx = file_index.fetch_add(1, Ordering::SeqCst) + 1;
                let fname = format!("{}_{:04}.{}", formatted_tags, idx, ext);
                let fpath = target_dir.join(&fname);
                if !fpath.exists() {
                    break (fname, fpath);
                }
            };

            if let Err(e) = tokio::fs::rename(&tmp_path, &final_filepath).await {
                eprintln!(
                    "Failed to rename {} to {}: {}",
                    tmp_path.display(),
                    final_filepath.display(),
                    e
                );
                let _ = tokio::fs::remove_file(&tmp_path).await;
                pb.finish_and_clear();
                return;
            }

            pb.finish_and_clear();
            println!("Downloaded: {}", final_filename);

            {
                let mut hist = history.lock().unwrap();
                hist.downloaded_ids.insert(post.id);
                if check_hash {
                    hist.file_hashes.insert(hash);
                }
                if !no_history {
                    let _ = save_history(&target_dir, &hist);
                }
            }
        });
    }

    while tasks.join_next().await.is_some() {}

    Ok(())
}
