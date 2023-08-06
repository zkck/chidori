use std::io;

use serde::Deserialize;
use serde::Serialize;

pub mod channel;
mod init;
pub mod message;

pub trait Handler<T> {
    fn process_message(
        &mut self,
        message: &message::Message<T>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Init {
    Init(init::Init),
    InitOk,
}

pub fn main_loop<TNode, TPayload>(node: &mut TNode) -> io::Result<()>
where
    TNode: Handler<TPayload>,
    for<'a> TPayload: Deserialize<'a>,
{
    let mut lines = io::stdin().lines();

    let first_message: message::Message<Init> =
        serde_json::from_str(&lines.next().expect("no value found in stdin")?).unwrap();

    let mut channel = match &first_message.body.payload {
        Init::Init(payload) => channel::MessageChannel::from(payload),
        _ => todo!(),
    };

    channel.reply(&first_message, &Init::InitOk).unwrap();

    for line in lines {
        let message: message::Message<TPayload> = serde_json::from_str(&line?).unwrap();
        node.process_message(&message, &mut channel).unwrap();
    }

    Ok(())
}
