use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Debug)]
pub struct UserGetterRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<&'a str>,
}

impl<'a> UserGetterRequest<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn login(mut self, login: &'a str) -> Self {
        self.login = Some(login);
        self
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UserGetterResponse {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: usize,
    pub created_at: String,
}
