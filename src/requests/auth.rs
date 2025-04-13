use crate::{
    errors::TokenError,
    models::info::{Bot, User},
};
use reqwest::header;

pub async fn validate_token(token: &str) -> Result<User, TokenError> {
    let token = "OAuth ".to_owned() + token;
    let response = reqwest::Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .header(header::AUTHORIZATION, token)
        .send()
        .await;
    let response: serde_json::Value = match response {
        Ok(response) if response.status() == 200 => response.json().await?,
        Ok(_) => return Err(TokenError::InvalidToken),
        Err(_) => return Err(TokenError::DeserializingError),
    };

    let user_id = response["user_id"].as_str();
    let user_nick = response["login"].as_str();

    let (user_id, user_nick) = match (user_id, user_nick) {
        (Some(user_id), Some(user_nick)) => (user_id.to_owned(), user_nick.to_owned()),
        _ => return Err(TokenError::DeserializingError),
    };
    let user = User { user_id, user_nick };
    Ok(user)
}

pub async fn validate_token_from_bot(bot: &Bot) -> Result<User, TokenError> {
    validate_token(&bot.access_token).await
}
