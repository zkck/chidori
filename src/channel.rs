use std::io;

use serde::Serialize;

use crate::init::Init;
use crate::message;

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
    pub fn reply<T, P>(
        &mut self,
        received: &message::Message<T>,
        payload: &P,
    ) -> Result<(), &'static str>
    where
        P: Serialize,
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

        serde_json::to_writer(io::stdout(), &reply_message).unwrap();
        println!();

        Ok(())
    }

    pub fn get_counter(&mut self) -> usize {
        let value = self.counter;
        self.counter += 1;
        value
    }
}
