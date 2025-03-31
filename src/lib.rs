use errors::{GettingDataError, TRequestsError};
use models::{
    file_variables::FileVariables,
    info::{Bot, User},
};

use requests::send_msg_request;
use token_getter::token_flow;
pub mod errors;
pub mod models;
pub mod requests;
pub mod token_getter;
mod util;

pub const URL: &str = "wss://eventsub.wss.twitch.tv/ws";

pub async fn open_file(filename: &str) -> Result<FileVariables, GettingDataError> {
    let string = tokio::fs::read_to_string(filename).await?;
    let file_variables = toml::from_str(&string)?;
    Ok(file_variables)
}

pub async fn get_token(file_variables: &FileVariables) -> Result<(User, Bot), TRequestsError> {
    let FileVariables {
        client_id,
        redirect_urls,
        command: _,
    } = file_variables;

    println!("Getting token");
    let access_token = token_flow::get_token(client_id, redirect_urls).await?;
    println!("Token received");

    println!("Validating token and getting user_id");
    let (user_id, user_nick) = token_flow::validate_token(&access_token).await?;
    println!("Valid token");

    let client_id = client_id.clone();

    let user = User { user_id, user_nick };
    let bot = Bot {
        client_id,
        access_token,
    };
    Ok((user, bot))
}

pub async fn subscribe_to_wb(
    bot_info: &Bot,
    session_id: &str,
    broadcaster_user_id: &str,
    user_id: &str,
) -> Result<(), TRequestsError> {
    println!("Starting subscription");
    requests::websocket_subscription(bot_info, session_id, broadcaster_user_id, user_id).await?;
    println!("Suscription to websocket succesfull");
    Ok(())
}

pub async fn send_msg(
    bot_info: &Bot,
    broadcaster_user_id: &str,
    user_id: &str,
    message: &str,
) -> Result<(), TRequestsError> {
    send_msg_request(bot_info, broadcaster_user_id, user_id, message).await?;
    Ok(())
}
