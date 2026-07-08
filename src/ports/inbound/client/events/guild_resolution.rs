#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum GuildResolution {
    Found(String),
    NotFound,
    Unknown,
}
