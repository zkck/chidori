use serde::Deserialize;
use serde::Serialize;

use crate::payloads;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub msg_id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: payloads::Payload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Message {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    pub body: MessageBody,
}
