use chidori;
use chidori::channel;
use chidori::message;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Payload {
    Broadcast {
        message: i64,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<i64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct Handler {
    messages: HashSet<i64>,
    topology: Option<HashMap<String, Vec<String>>>,
}

impl chidori::Handler<Payload> for Handler {
    fn process_message(
        &mut self,
        received: &message::Message<Payload>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        match &received.body.payload {
            Payload::Broadcast { message } => {
                self.messages.insert(*message);
                channel.reply(received, &Payload::BroadcastOk)?
            }
            Payload::Read => channel.reply(
                received,
                &Payload::ReadOk {
                    messages: self.messages.clone(),
                },
            )?,
            Payload::Topology { topology } => {
                self.topology = Some(topology.clone());
                channel.reply(received, &Payload::TopologyOk)?
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut handler = Handler {
        messages: HashSet::new(),
        topology: None,
    };
    chidori::main_loop(&mut handler)
}
