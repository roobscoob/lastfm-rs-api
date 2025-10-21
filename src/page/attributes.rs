use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde_as(as = "DisplayFromStr")]
    pub total_pages: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub page: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub per_page: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
}
