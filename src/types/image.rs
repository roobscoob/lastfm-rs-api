use std::sync::Arc;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub size: ImageSize,

    #[serde(default)]
    pub url: Option<Arc<str>>,
}
