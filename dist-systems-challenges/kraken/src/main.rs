use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct MessageBody {
    #[serde(alias = "type")]
    #[serde(rename(serialize = "type"))]
    msg_type: String,
    msg_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

struct Node {
    node_id: Option<String>,
    message_handles: Option<Vec<Box<dyn Fn(String, Message) -> Option<Message>>>>,
    in_messages_send: Sender<Message>,
    in_messages_recv: Receiver<Message>,
}

impl Default for Node {
    fn default() -> Self {
        let (in_messages_send, in_messages_recv) = mpsc::channel::<Message>();
        Node {
            in_messages_send,
            in_messages_recv,
            node_id: None,
            message_handles: None,
        }
    }
}

impl Node {
    pub fn handle_message(&mut self, f: Box<dyn Fn(String, Message) -> Option<Message>>) {
        if let Some(message_handles) = self.message_handles.as_mut() {
            message_handles.push(f);
        } else {
            self.message_handles = Some(vec![f]);
        }
    }

    pub fn run(&mut self) {
        self.process_in();
        self.process_out();
    }

    fn process_in(&self) {
        let sender = self.in_messages_send.clone();
        thread::spawn(move || loop {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Could not read line.");
            match serde_json::from_str::<Message>(&line) {
                Ok(value) => {
                    if let Err(_) = sender.send(value) {
                        eprintln!("Unable to send message");
                    };
                }
                Err(err) => {
                    eprintln!("Unable to parse json to data type Message.\nError: {}", err);
                }
            }
        });
    }

    fn send_message(msg: Message) {
        if let Ok(msg_json) = serde_json::to_string(&msg) {
            eprintln!("Sending message: {}", msg_json);
            println!("{}", msg_json);
        } else {
            eprintln!("Unable to format message to json string.");
        }
    }

    fn process_out(&mut self) {
        loop {
            if let Ok(msg) = self.in_messages_recv.recv() {
                if msg.body.msg_type == "init" {
                    self.handle_message_in(msg);
                } else {
                    if let Some(message_handles) = self.message_handles.as_ref() {
                        let msg_out = msg.clone();
                        let _ = message_handles
                            .iter()
                            .map(move |f| {
                                if let Some(m) = f(msg_out.body.msg_type.clone(), msg_out.clone()) {
                                    eprintln!("Sending message...");
                                    Node::send_message(m);
                                }
                            })
                            .collect::<()>();
                    }
                }
            }
        }
    }

    fn handle_message_in(&mut self, msg: Message) {
        if let Some(id) = msg.body.node_id {
            self.node_id = Some(id.clone());
            Node::send_message(Message {
                src: id,
                dest: msg.src,
                body: MessageBody {
                    msg_type: "init_ok".to_string(),
                    msg_id: msg.body.msg_id,
                    in_reply_to: Some(msg.body.msg_id),
                    ..Default::default()
                },
            });
        }
    }
}

fn main() {
    let mut node: Node = Default::default();
    node.handle_message(Box::new(|msg_type, mut msg| {
        if msg_type == "echo" {
            let dest = msg.dest;
            msg.dest = msg.src;
            msg.src = dest;
            msg.body.in_reply_to = Some(msg.body.msg_id);
            msg.body.msg_type = "echo_ok".to_string();
        }
        Some(msg)
    }));
    node.run();
}
