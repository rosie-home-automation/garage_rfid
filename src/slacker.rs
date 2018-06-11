use slack_hook::{Slack, PayloadBuilder, Payload};
use slog;

use configuration::Configuration;

#[derive(Debug)]
pub struct Slacker {
  channel: String,
  webhook_url: String,
  username: String,
}

impl Slacker {
  pub fn new(configuration: &Configuration, logger: slog::Logger) -> Slacker {
    let channel = configuration.slack.channel.clone();
    let webhook_url = configuration.slack.webhook_url.clone();
    let username = configuration.slack.username.clone();
    let slacker = Slacker {
      channel: channel,
      webhook_url: webhook_url,
      username: username,
    };
    info!(logger, "Initialized slacker."; "slacker" => ?slacker);
    slacker
  }

  pub fn send_text(&self, text: &str, logger: slog::Logger) {
    let message = self.build_message(text);
    let slack = Slack::new(self.webhook_url.as_str()).unwrap();
    let response = slack.send(&message);
    match response {
      Ok(()) => info!(logger, "Sent message to slack."; "message" => ?message),
      Err(err) => error!(logger, "Failed to send message to slack"; "error" => ?err,
        "message" => ?message),
    }
  }

  fn build_message(&self, text: &str) -> Payload {
    PayloadBuilder::new()
      .channel(self.channel.as_str())
      .username(self.username.as_str())
      .text(text)
      .build()
      .unwrap()
  }
}
