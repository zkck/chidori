use std::io;

mod message;
mod payloads;

struct NodeInfo {
    node_id: String,
    _node_ids: Vec<String>,
}

struct Node {
    node_info: Option<NodeInfo>,
    current_id: usize,
}

impl Node {
    fn node_id(&self) -> Option<String> {
        match &self.node_info {
            Some(node_info) => Some(node_info.node_id.clone()),
            None => None,
        }
    }

    fn reply(
        &mut self,
        message: &message::Message,
        payload: payloads::Payload,
    ) -> Result<message::Message, &'static str> {
        Ok(message::Message {
            src: self.node_id().ok_or("node not initialized")?,
            dest: message.src.clone(),
            body: message::MessageBody {
                msg_id: Some(self.get_msg_id()),
                in_reply_to: message.body.msg_id,
                payload,
            },
        })
    }

    fn handle(&mut self, message: &message::Message) -> Result<message::Message, &'static str> {
        match &message.body.payload {
            payloads::Payload::Init { node_id, node_ids } => {
                self.node_info = Some(NodeInfo {
                    node_id: node_id.clone(),
                    _node_ids: node_ids.clone(),
                });
                self.reply(message, payloads::Payload::InitOk)
            }
            payloads::Payload::Echo { echo } => {
                self.reply(message, payloads::Payload::EchoOk { echo: echo.clone()})
            },
            _ => Err("unsupported message type"),
        }
    }

    fn get_msg_id(&mut self) -> usize {
        let msg_id = self.current_id;
        self.current_id += 1;
        msg_id
    }
}

fn main() -> io::Result<()> {
    let mut node = Node {
        node_info: None,
        current_id: 0,
    };

    for line in io::stdin().lines() {
        match serde_json::from_str(&line?) {
            Ok(message) => match node.handle(&message) {
                Ok(answer) => {
                    serde_json::to_writer(io::stdout(), &answer)?;
                    println!();
                }
                Err(error) => eprintln!("Error handling message: {}", error),
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_echo() {
        let mut node = Node {
            node_info: Some(NodeInfo {
                node_id: "n42".to_string(),
                _node_ids: vec![],
            }),
            current_id: 0,
        };

        let message = message::Message {
            src: "n0".to_owned(),
            dest: "n42".to_owned(),
            body: message::MessageBody {
                msg_id: Some(1234),
                in_reply_to: None,
                payload: payloads::Payload::Echo {
                    echo: "Bonjour".to_string(),
                },
            },
        };

        let expected = message::Message {
            src: "n42".to_owned(),
            dest: "n0".to_owned(),
            body: message::MessageBody {
                msg_id: Some(0),
                in_reply_to: Some(1234),
                payload: payloads::Payload::EchoOk {
                    echo: "Bonjour".to_string(),
                },
            },
        };

        assert_eq!(Ok(expected), node.handle(&message))
    }
}
