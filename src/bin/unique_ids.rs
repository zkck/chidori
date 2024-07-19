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
    Generate,
    GenerateOk { id: String },
}

struct Handler {
    counter: usize,
}

impl chidori::Handler<Payload> for Handler {
    fn handle_message(
        &mut self,
        message: &message::Message<Payload>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        let id = format!("{}{}", channel.node_id, self.counter);
        self.counter += 1;

        if let Payload::Generate = message.body.payload {
            channel.reply(&message, &Payload::GenerateOk { id })?
        }

        Ok(())
    }

    fn handle_tick(&mut self, _channel: &mut channel::MessageChannel) -> Result<(), &'static str> {
        Ok(())
    }

    fn send_events(&self, _send_channel: &std::sync::mpsc::Sender<chidori::Event>) {}
}

fn main() -> io::Result<()> {
    let mut handler = Handler { counter: 0 };
    chidori::main_loop(&mut handler)
}
