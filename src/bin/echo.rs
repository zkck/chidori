use chidori;
use chidori::channel;
use chidori::message;
use serde::Deserialize;
use serde::Serialize;

use std::io;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct Handler;

impl chidori::Handler<Payload> for Handler {
    fn process_message(
        &mut self,
        message: &message::Message<Payload>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        if let Payload::Echo { echo } = &message.body.payload {
            channel.reply(message, &Payload::EchoOk { echo: echo.clone() })?
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut handler = Handler {};
    chidori::main_loop(&mut handler)
}
