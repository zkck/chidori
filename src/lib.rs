use std::io;
use std::sync::mpsc;
use std::thread;

use serde::Deserialize;
use serde::Serialize;

pub mod channel;
mod init;
pub mod message;

pub enum Event {
    Message(String),
    Tick,
}

pub trait Handler<T> {
    fn handle_message(
        &mut self,
        message: &message::Message<T>,
        channel: &mut channel::MessageChannel,
    ) -> Result<(), &'static str>;

    fn handle_tick(&mut self, channel: &mut channel::MessageChannel) -> Result<(), &'static str>;

    fn send_events(&self, send_channel: &mpsc::Sender<Event>);
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Init {
    Init(init::Init),
    InitOk,
}

fn send_events_from_stdin(tx: &mpsc::Sender<Event>) {
    let tx = tx.clone();
    thread::spawn(move || {
        let lines = io::stdin().lines();
        for line in lines {
            tx.send(Event::Message(line.unwrap())).unwrap();
        }
    });
}

fn create_channel_from_init() -> io::Result<channel::MessageChannel> {
    let mut lines = io::stdin().lines();

    let first_message: message::Message<Init> =
        serde_json::from_str(&lines.next().expect("no value found in stdin")?).unwrap();

    let mut channel = match &first_message.body.payload {
        Init::Init(payload) => channel::MessageChannel::from(payload),
        _ => todo!(),
    };

    channel.reply(&first_message, &Init::InitOk).unwrap();
    Ok(channel)
}

pub fn main_loop<TNode, TPayload>(node: &mut TNode) -> io::Result<()>
where
    TNode: Handler<TPayload>,
    for<'a> TPayload: Deserialize<'a> + Send,
{
    let mut message_channel = create_channel_from_init()?;

    let (tx, rx) = mpsc::channel();
    send_events_from_stdin(&tx);
    node.send_events(&tx);

    for event in rx {
        match event {
            Event::Message(string) => {
                let message = serde_json::from_str(&string).unwrap();
                node.handle_message(&message, &mut message_channel).unwrap();
            }
            Event::Tick => {
                node.handle_tick(&mut message_channel).unwrap();
            }
        };
    }
    Ok(())
}
