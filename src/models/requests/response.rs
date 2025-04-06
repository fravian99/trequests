use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct UnpagedResponse<T> {
    pub data: Vec<T>,
}
