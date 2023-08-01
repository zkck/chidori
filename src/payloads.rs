use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
    Read { key: usize },
    ReadOk { value: usize },
}
