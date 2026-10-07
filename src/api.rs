use crate::config::Config;
use crate::history::History;
use serde::Deserialize;
use std::path::Path;
use url::form_urlencoded;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponse {
    #[serde(default)]
    pub post: Option<Vec<Post>>,
    #[serde(default)]
    pub posts: Option<Vec<Post>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub id: u64,
    pub file_url: String,
}

pub fn prepare_tags(raw_tags: &str, config: &Config) -> Result<(String, String), String> {
    let cli_tags: Vec<&str> = raw_tags.split_whitespace().collect();

    let (mut neg_tags, pos_tags): (Vec<String>, Vec<String>) = cli_tags
        .into_iter()
        .map(|s| s.to_string())
        .partition(|tag| tag.starts_with('-'));

    if pos_tags.is_empty() {
        return Err("You must provide at least one positive tag.".to_string());
    }

    let mut global_neg_tags = config.global_negative_tags.clone();
    if config.sort_negative_tags.unwrap_or(true) {
        global_neg_tags.sort();
    }

    for mut global_tag in global_neg_tags {
        if !global_tag.starts_with('-') {
            global_tag.insert(0, '-');
        }
        if !neg_tags.contains(&global_tag) {
            neg_tags.push(global_tag);
        }
    }

    if config.sort_negative_tags.unwrap_or(true) {
        neg_tags.sort();
    }

    let encoded_pos_tags: Vec<String> = pos_tags.iter().map(|t| url_encode_tag(t)).collect();
    let encoded_neg_tags: Vec<String> = neg_tags.iter().map(|t| url_encode_tag(t)).collect();

    let formatted_tags = pos_tags.join("+");
    let api_formatted_tags = encoded_pos_tags.join("+");

    let api_tags = if encoded_neg_tags.is_empty() {
        api_formatted_tags
    } else {
        format!("{}+{}", api_formatted_tags, encoded_neg_tags.join("+"))
    };

    Ok((formatted_tags, api_tags))
}

fn url_encode_tag(tag: &str) -> String {
    form_urlencoded::byte_serialize(tag.as_bytes()).collect()
}

pub async fn fetch_posts(
    client: &reqwest::Client,
    config: &Config,
    api_tags: &str,
    limit: usize,
    skip_gif: bool,
    history: &History,
) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let allowed_exts = ["jpg", "jpeg", "png", "gif", "webm", "mp4"];
    let mut all_valid_posts: Vec<Post> = Vec::new();
    let mut pid = 0;
    let api_limit = 100;

    while all_valid_posts.len() < limit {
        let url = format!(
            "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit={}&pid={}&tags={}&user_id={}&api_key={}",
            api_limit, pid, api_tags, config.user_id, config.api_key
        );

        let res = client.get(&url).send().await?;

        if !res.status().is_success() {
            return Err(format!("Encountered an error with code: {}", res.status()).into());
        }

        let text_res = res.text().await?;

        if text_res.trim().is_empty() || text_res == "[]" {
            break;
        }

        let posts: Vec<Post> = if let Ok(api_res) = serde_json::from_str::<ApiResponse>(&text_res) {
            api_res.posts.or(api_res.post).unwrap_or_default()
        } else if let Ok(direct_vec) = serde_json::from_str::<Vec<Post>>(&text_res) {
            direct_vec
        } else {
            break;
        };

        if posts.is_empty() {
            break;
        }

        let posts_len = posts.len();

        for post in posts {
            if all_valid_posts.len() >= limit {
                break;
            }

            if history.downloaded_ids.contains(&post.id) {
                continue;
            }

            if !(post.file_url.starts_with("http://") || post.file_url.starts_with("https://")) {
                continue;
            }

            let ext = Path::new(&post.file_url)
                .extension()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("bin")
                .to_lowercase();

            if !allowed_exts.contains(&ext.as_str()) {
                continue;
            }

            if skip_gif && ext == "gif" {
                continue;
            }

            all_valid_posts.push(post);
        }

        pid += 1;

        if posts_len < api_limit {
            break;
        }
    }

    Ok(all_valid_posts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_tags_requires_positive_tag() {
        let config = Config::default();
        let result = prepare_tags("-tag1 -tag2", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_tags_success() {
        let config = Config::default();
        let (formatted, api_tags) = prepare_tags("cat dog -bad", &config).unwrap();
        assert_eq!(formatted, "cat+dog");
        assert!(api_tags.contains("cat+dog"));
        assert!(api_tags.contains("-3d"));
        assert!(api_tags.contains("-bad"));
        assert!(api_tags.contains("-low_res"));
    }
}
