use core::str;

use crate::{
    TRequestsResult,
    models::{
        info::Bot,
        requests::{
            response::UnpagedResponse,
            send_msg_request::SendMsgRequest,
            user_getter::{UserGetterRequest, UserGetterResponse},
            wb_subscription::EventSubRequestListenerBuilder,
        },
    },
};

pub async fn websocket_subscription(
    bot_info: &Bot,
    session_id: &str,
    broadcaster_user_id: &str,
    user_id: &str,
) -> Result<(), reqwest::Error> {
    let body = EventSubRequestListenerBuilder::new()
        .type_param("channel.chat.message")
        .version("1")
        .broadcaster_user_id(broadcaster_user_id)
        .user_id(user_id)
        .method("websocket")
        .session_id(session_id)
        .build();

    let request = reqwest::Client::new()
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .json(&body);

    let response = request.send().await?;
    response.error_for_status()?;
    Ok(())
}

pub async fn send_msg_request(
    bot_info: &Bot,
    msg_request: &SendMsgRequest<'_>,
) -> Result<(), reqwest::Error> {
    let request = reqwest::Client::new()
        .post("https://api.twitch.tv/helix/chat/messages")
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .json(msg_request);

    let response = request.send().await?;
    response.error_for_status()?;
    Ok(())
}

pub async fn get_users(
    bot_info: &Bot,
    user: &UserGetterRequest<'_>,
) -> TRequestsResult<Vec<UserGetterResponse>> {
    let url = "https://api.twitch.tv/helix/users";

    let request = reqwest::Client::new()
        .get(url)
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .query(user);
    let response = request.send().await?;
    let users_getter_response: UnpagedResponse<UserGetterResponse> = response.json().await?;
    Ok(users_getter_response.data)
}
