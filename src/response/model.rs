use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Text,
    Application,
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseSubtype {
    Html,
    Json,
    Plain,
    Xml,
    Unknown,
}
