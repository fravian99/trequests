use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct UnpagedResponse<T> {
    pub data: Vec<T>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Pagination {
    pub cursor: String,
}
