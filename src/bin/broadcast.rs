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
    fn handle_message(
        &mut self,
        received: &message::Message<Payload>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        match &received.body.payload {
            Payload::Broadcast { message } => {
                if self.messages.insert(*message) {
                    // new message, propagate
                    let neighbors = self
                        .topology
                        .as_ref()
                        .and_then(|t| t.get(&channel.node_id))
                        .ok_or("unknown topology")?
                        .clone();
                    for neighbor in &neighbors {
                        channel.send(
                            neighbor,
                            &Payload::Broadcast {
                                message: message.clone(),
                            },
                        )?;
                    }
                }
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

    fn handle_tick(&mut self, _channel: &mut channel::MessageChannel) -> Result<(), &'static str> {
        // does nothing
        Ok(())
    }

    fn send_events(&self, _send_channel: &std::sync::mpsc::Sender<chidori::Event>) {
        // does nothing
    }
}

fn main() -> io::Result<()> {
    let mut handler = Handler {
        messages: HashSet::new(),
        topology: None,
    };
    chidori::main_loop(&mut handler)
}
