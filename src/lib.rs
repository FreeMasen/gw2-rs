use uuid::Uuid;

pub mod client;
pub mod codec;
pub mod endpoints;

trait UpperHyphenated {
    fn as_upper_hyphenated(&self) -> String;
}

impl UpperHyphenated for Uuid {
    fn as_upper_hyphenated(&self) -> String {
        format!("{}", self.as_hyphenated()).to_ascii_uppercase()
    }
}

#[derive(Debug, Clone)]
pub struct PagedResult<T> {
    pub page: PageInfo,
    pub result: Vec<T>,
}

#[derive(Debug, Clone, Default)]
pub struct PageInfo {
    pub links: Links,
    pub total_pages: Option<usize>,
    pub page_size: Option<usize>,
    pub result_count: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Links {
    next: Option<Link>,
    first: Option<Link>,
    this: Option<Link>,
    last: Option<Link>,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    WrappedReqwest(#[from] reqwest_middleware::Error),
    #[error("Reached the rate limit on this request")]
    RateLimit,
    #[error("Request Timed Out")]
    Timeout,
    #[error("Unknown error")]
    Unknown,
}
