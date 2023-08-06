use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MessageBody<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub msg_id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Message<T> {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    pub body: MessageBody<T>,
}
