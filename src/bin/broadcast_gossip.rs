use chidori;
use chidori::channel;
use chidori::message;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
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
    // Custom message
    Gossip {
        message: i64,
    },
}

struct Handler {
    messages: HashSet<i64>,
    topology: Option<HashMap<String, Vec<String>>>,

    rng: ThreadRng,
}

impl chidori::Handler<Payload> for Handler {
    fn process_message(
        &mut self,
        received: &message::Message<Payload>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        match &received.body.payload {
            Payload::Broadcast { message } => {
                self.gossip(*message, channel)?;
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
            Payload::Gossip { message } => {
                self.gossip(*message, channel)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl Handler {
    fn get_neighbors(&self, node_id: &str) -> Option<Vec<String>> {
        self.topology.as_ref().and_then(|t| t.get(node_id)).cloned()
    }
    fn gossip(
        &mut self,
        message: i64,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str> {
        if self.messages.insert(message) {
            for neighbor in self
                .get_neighbors(&channel.node_id)
                .ok_or("no neighbors")?
                .choose_multiple(&mut self.rng, 5)
            {
                channel.send(
                    neighbor,
                    &Payload::Gossip {
                        message: message.clone(),
                    },
                )?;
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut handler = Handler {
        messages: HashSet::new(),
        topology: None,
        rng: rand::thread_rng(),
    };
    chidori::main_loop(&mut handler)
}
