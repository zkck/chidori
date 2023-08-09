use chidori;
use chidori::channel;
use chidori::message;
use chidori::Event;
use rand::seq::SliceRandom;
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

    known_by_dest: HashMap<String, HashSet<i64>>,
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
                self.known_by_dest
                    .entry(received.src.clone())
                    .or_default()
                    .extend(messages);
                // no reply
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_tick(&mut self, channel: &mut channel::MessageChannel) -> Result<(), &'static str> {
        let Some(neighbors) = self.get_neighbors(&channel.node_id) else {
            // topology not yet received, do not gossip
            return Ok(())
        };

        let mut rng = rand::thread_rng();

        for neighbor in neighbors {
            let (known, mut unknown) = self.messages.iter().partition::<Vec<i64>, _>(|m| {
                self.known_by_dest
                    .entry(neighbor.clone())
                    .or_default()
                    .contains(m)
            });
            // TODO: Explanation
            unknown.extend(known.as_slice().choose_multiple(&mut rng, 5));
            channel.send(
                &neighbor,
                &Payload::Gossip {
                    messages: HashSet::from_iter(unknown),
                },
            )?;
        }
        Ok(())
    }

    fn send_events(&self, send_channel: &mpsc::Sender<chidori::Event>) {
        let send_channel = send_channel.clone();
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(50));
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

        known_by_dest: HashMap::new(),
    };
    chidori::main_loop(&mut handler)
}
