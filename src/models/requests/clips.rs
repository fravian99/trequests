use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug)]
pub struct ClipRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    broadcaster_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ended_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_featured: Option<bool>,
}

impl<'a> ClipRequest<'a> {
    pub fn builder() -> ClipBuilder<'a> {
        ClipBuilder::default()
    }
}

#[derive(Default)]
pub struct ClipBuilder<'a> {
    broadcaster_id: Option<&'a str>,
    game_id: Option<&'a str>,
    id: Option<&'a str>,
    started_at: Option<&'a str>,
    ended_at: Option<&'a str>,
    first: Option<usize>,
    before: Option<&'a str>,
    after: Option<&'a str>,
    is_featured: Option<bool>,
}
impl<'a> ClipBuilder<'a> {
    pub fn broadcaster_id(&mut self, broadcaster_id: &'a str) -> &mut Self {
        self.broadcaster_id = Some(broadcaster_id);
        self
    }

    pub fn game_id(&mut self, game_id: &'a str) -> &mut Self {
        self.game_id = Some(game_id);
        self
    }

    pub fn id(&mut self, id: &'a str) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn started_at(&mut self, started_at: &'a str) -> &mut Self {
        self.started_at = Some(started_at);
        self
    }

    pub fn ended_at(&mut self, ended_at: &'a str) -> &mut Self {
        self.ended_at = Some(ended_at);
        self
    }

    pub fn first(&mut self, first: usize) -> &mut Self {
        self.first = Some(first);
        self
    }

    pub fn before(&mut self, before: &'a str) -> &mut Self {
        self.before = Some(before);
        self
    }

    pub fn after(&mut self, after: &'a str) -> &mut Self {
        self.after = Some(after);
        self
    }

    pub fn is_featured(&mut self, is_featured: bool) -> &mut Self {
        self.is_featured = Some(is_featured);
        self
    }

    fn validate(&mut self) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
        match (self.broadcaster_id, self.game_id, self.id) {
            (Some(broadcaster_id), None, None) => (Some(broadcaster_id), None, None),
            (None, Some(game_id), None) => (None, Some(game_id), None),
            (None, None, Some(id)) => (None, None, Some(id)),
            (None, None, None) => {
                panic!("broadcaster_id, game_id and id can not be None at the same time")
            }
            _ => panic!("broadcaster_id, game_id and id are mutually exclusive"),
        }
    }

    pub fn build(&mut self) -> ClipRequest<'a> {
        let (broadcaster_id, game_id, id) = self.validate();
        ClipRequest {
            broadcaster_id,
            game_id,
            id,
            started_at: self.started_at,
            ended_at: self.ended_at,
            first: self.first,
            before: self.before,
            after: self.after,
            is_featured: self.is_featured,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipResponse {
    pub id: String,
    pub url: String,
    pub embed_url: String,
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub creator_id: String,
    pub creator_name: String,
    pub video_id: String,
    pub game_id: String,
    pub language: String,
    pub title: String,
    pub view_count: usize,
    pub created_at: String,
    pub thumbnail_url: String,
    pub duration: f64,
    pub vod_offset: Option<usize>,
}

#[cfg(test)]
mod test {
    use super::ClipRequest;

    #[test]
    pub fn test_build() {
        let clip = ClipRequest::builder().broadcaster_id("a").build();
        assert_eq!(clip.broadcaster_id, Some("a"));
        let clip = ClipRequest::builder().game_id("b").build();
        assert_eq!(clip.game_id, Some("b"));
        let clip = ClipRequest::builder().id("c").build();
        assert_eq!(clip.id, Some("c"));
    }

    #[test]
    #[should_panic(expected = "broadcaster_id, game_id and id can not be None at the same time")]
    pub fn test_none() {
        ClipRequest::builder().build();
    }

    #[test]
    #[should_panic(expected = "broadcaster_id, game_id and id are mutually exclusive")]
    pub fn test_exclusive() {
        ClipRequest::builder()
            .broadcaster_id("a")
            .game_id("a")
            .id("a")
            .build();
    }
}
