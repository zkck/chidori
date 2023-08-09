use chidori;
use chidori::channel;
use chidori::message;
use chidori::Event;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

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
    // Custom message
    Gossip {
        messages: HashSet<i64>,
    },
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
            Payload::Gossip { messages } => {
                self.messages.extend(messages);
                // no reply
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_tick(&mut self, channel: &mut channel::MessageChannel) -> Result<(), &'static str> {
        let Some(neighbors) = self.get_neighbors(&channel.node_id)
        else {
            return Ok(())
        };
        for neighbor in neighbors {
            channel.send(
                &neighbor,
                &Payload::Gossip {
                    messages: self.messages.clone(),
                },
            )?;
        }
        Ok(())
    }

    fn send_events(&self, send_channel: &mpsc::Sender<chidori::Event>) {
        let send_channel = send_channel.clone();
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(200));
            send_channel.send(Event::Tick).unwrap();
        });
    }
}
impl Handler {
    fn get_neighbors(&self, node_id: &str) -> Option<Vec<String>> {
        self.topology.as_ref().and_then(|t| t.get(node_id)).cloned()
    }
}

fn main() -> io::Result<()> {
    let mut handler = Handler {
        messages: HashSet::new(),
        topology: None,
    };
    chidori::main_loop(&mut handler)
}
