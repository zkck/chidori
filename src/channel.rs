use std::io;

use serde::Serialize;

use crate::init::Init;
use crate::message;

fn write<T>(message: &message::Message<T>)
where
    T: Serialize,
{
    serde_json::to_writer(io::stdout(), &message).unwrap();
    println!();
}

pub struct MessageChannel {
    pub node_id: String,
    pub node_ids: Vec<String>,

    counter: usize,
}

impl From<&Init> for MessageChannel {
    fn from(value: &Init) -> Self {
        Self {
            node_id: value.node_id.clone(),
            node_ids: value.node_ids.clone(),
            counter: 0,
        }
    }
}

impl MessageChannel {
    pub fn send<T>(&mut self, node: &str, payload: &T) -> Result<(), &'static str>
    where
        T: Serialize,
    {
        let reply_message = message::Message {
            src: self.node_id.clone(),
            dest: node.to_string(),
            body: message::MessageBody {
                msg_id: Some(self.get_counter()),
                in_reply_to: None,
                payload,
            },
        };

        write(&reply_message);
        Ok(())
    }

    pub fn reply<T>(
        &mut self,
        received: &message::Message<T>,
        payload: &T,
    ) -> Result<(), &'static str>
    where
        T: Serialize,
    {
        let reply_message = message::Message {
            src: self.node_id.clone(),
            dest: received.src.clone(),
            body: message::MessageBody {
                msg_id: Some(self.get_counter()),
                in_reply_to: received.body.msg_id,
                payload,
            },
        };

        write(&reply_message);
        Ok(())
    }

    fn get_counter(&mut self) -> usize {
        let value = self.counter;
        self.counter += 1;
        value
    }
}
