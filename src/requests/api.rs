use crate::{
    TRequestsResult,
    models::{
        info::Bot,
        requests::{
            clips::{ClipRequest, ClipResponse},
            response::{PagedResponse, Pagination, UnpagedResponse},
            send_msg_request::SendMsgRequest,
            user_getter::{UserGetterRequest, UserGetterResponse},
        },
    },
};

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

impl SendMsgRequest<'_> {
    pub async fn send(&self, bot_info: &Bot) -> Result<(), reqwest::Error> {
        send_msg_request(bot_info, self).await
    }
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

impl UserGetterRequest<'_> {
    pub async fn get(&self, bot_info: &Bot) -> TRequestsResult<Vec<UserGetterResponse>> {
        get_users(bot_info, self).await
    }
}

pub async fn get_clips(
    bot_info: &Bot,
    clip: &ClipRequest<'_>,
) -> TRequestsResult<(Vec<ClipResponse>, Pagination)> {
    let request = reqwest::Client::new()
        .get("https://api.twitch.tv/helix/clips")
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .query(clip);

    let response = request.send().await?;
    let clips: PagedResponse<ClipResponse> = response.json().await?;
    Ok((clips.data, clips.pagination))
}

impl ClipRequest<'_> {
    pub async fn get(&self, bot_info: &Bot) -> TRequestsResult<(Vec<ClipResponse>, Pagination)> {
        get_clips(bot_info, self).await
    }
}
