use crate::Discord;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Notifier {
    fn name(&self) -> &'static str;
    async fn notify(&self, diff: &str) -> Result<(), String>;
}

fn init_notifiers() -> Vec<Box<dyn Notifier>> {
    let mut notifiers: Vec<Box<dyn Notifier>> = Vec::new();

    if let Ok(discord_webhook_url) = std::env::var("DISCORD_WEBHOOK_URL") {
        let user_id = std::env::var("DISCORD_USER_ID").ok();
        let message = std::env::var("DISCORD_MESSAGE").ok();

        notifiers.push(Box::new(Discord {
            webhook_url: discord_webhook_url,
            user_id,
            message,
        }));
    }

    notifiers.iter().for_each(|notifier| {
        println!("{} notifier initialized", notifier.name());
    });

    notifiers
}

pub(crate) async fn notify_all(diff: &str) {
    let notifiers = init_notifiers();

    for notifier in notifiers {
        match notifier.notify(diff).await {
            Ok(_) => println!("Notified via {}", notifier.name()),
            Err(error) => eprintln!("Failed to notify via {}: {}", notifier.name(), error),
        }
    }
}
