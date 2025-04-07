use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SendMsgRequest<'a> {
    pub broadcaster_id: &'a str,
    pub sender_id: &'a str,
    pub message: String,
    #[serde(
        rename = "reply_parent_message_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to: Option<&'a str>,
}

impl<'a> SendMsgRequest<'a> {
    pub fn new(broadcaster_id: &'a str, sender_id: &'a str, message: &'a str) -> Self {
        let message = message.to_owned();
        Self {
            broadcaster_id,
            sender_id,
            message,
            reply_to: None,
        }
    }

    pub fn reply_to(mut self, reply_to: &'a str) -> Self {
        self.reply_to = Some(reply_to);
        self
    }

    pub fn bot_name(mut self, bot_name: &'a str) -> Self {
        self.message = format!("[{}]: {}", bot_name, self.message);
        self
    }
}
