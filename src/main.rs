extern crate reqwest;

mod discord;
mod notifier;

use discord::Discord;
use dotenv::dotenv;
use similar::{group_diff_ops, ChangeTag, TextDiff};
use std::fs;

async fn fetch_contents(url: &str) -> Result<String, reqwest::Error> {
    let user_agent = std::env::var("USER_AGENT").unwrap_or("reqwest".to_string());
    let client = reqwest::Client::builder().user_agent(user_agent).build()?;
    return client.get(url).send().await?.text().await;
}

fn without_ignored_lines(body: &str, ignored_lines: &Vec<&str>) -> String {
    body.lines()
        .filter(|line| ignored_lines.iter().all(|ignored| !line.contains(ignored)))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn diff(old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    let mut result = String::new();

    let group_size: usize = std::env::var("GROUP_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);

    for op in group_diff_ops(diff.ops().to_vec(), group_size) {
        for group in op {
            for change in diff.iter_changes(&group) {
                let tag = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                result.push_str(format!("{} {}", tag, change).as_str())
            }
        }
    }

    result
}

#[tokio::main]
async fn main() {
    if let Err(_) = dotenv() {
        eprintln!("Failed to load .env file");
    }

    let url = std::env::var("URL");
    let ignored_lines_str = std::env::var("IGNORED_LINES").unwrap_or("".to_string());
    let ignored_lines = ignored_lines_str.split(",").collect::<Vec<&str>>();

    match url {
        Ok(url) => {
            let url_path_safe = url.replace(|c: char| !c.is_ascii_alphanumeric(), "");

            let output_dir = std::env::var("OUTPUT_DIR").unwrap_or("./outputs".to_string());
            fs::create_dir_all(&output_dir).unwrap();

            let output_file = format!("{}/{}.html", output_dir, url_path_safe);

            let res = fetch_contents(&url).await;
            let body = res.unwrap();
            let body_without_ignored_lines = without_ignored_lines(&body, &ignored_lines);

            match fs::read_to_string(&output_file) {
                Ok(old_body) => {
                    let old_body_without_ignored_lines =
                        without_ignored_lines(&old_body, &ignored_lines);
                    if old_body_without_ignored_lines != body_without_ignored_lines {
                        fs::write(&output_file, &body).unwrap();
                        notifier::notify_all(&diff(
                            &old_body_without_ignored_lines,
                            &body_without_ignored_lines,
                        ))
                        .await;
                    }
                }
                Err(_) => {
                    fs::write(&output_file, body).unwrap();
                }
            }
        }
        Err(_) => {
            eprintln!("Please specify a valid URL in the URL environment variable.");
            std::process::exit(1);
        }
    }
}
