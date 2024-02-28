extern crate reqwest;

use crate::notifier::Notifier;
use async_trait::async_trait;
use serde_json::json;

const CHARACTER_LIMIT: usize = 2000;

pub(crate) struct Discord {
    pub webhook_url: String,
    pub user_id: Option<String>,
    pub message: Option<String>,
}

#[async_trait]
impl<'a> Notifier for Discord {
    fn name(&self) -> &'static str {
        "Discord"
    }

    async fn notify(&self, diff: &str) -> Result<(), String> {
        let tag: Option<String> = self
            .user_id
            .to_owned()
            .and_then(|user_id| Some(format!("<@{}>", user_id)));

        // 2 is for the newline
        let tag_length = tag.as_ref().map_or(0, |s| s.len() + 2);
        let message_length = self.message.as_ref().map_or(0, |s| s.len() + 2);

        // 14 is for the code block and the newlines
        let characters_left = CHARACTER_LIMIT - tag_length - message_length - 14;
        let diff = if diff.len() > characters_left {
            &diff[..characters_left]
        } else {
            diff
        };
        let formatted_diff = format!("```diff\n{}\n```", diff);

        let content_vec: Vec<Option<String>> =
            vec![tag, self.message.to_owned(), Some(formatted_diff)];
        let content = content_vec
            .into_iter()
            .filter_map(|s| s)
            .collect::<Vec<String>>()
            .join("\n");

        let client = reqwest::Client::new();
        let res = client
            .post(self.webhook_url.as_str())
            .json(&json!({ "content": content }))
            .send()
            .await;

        match res.and_then(|r| r.error_for_status()) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Failed to send a message to Discord: {}", error)),
        }
    }
}
