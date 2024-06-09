use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum MessageType {
    Prepare,
    Promise,
    Propose,
    Accept,
    Accepted,
}

#[derive(Clone)]
struct Message {
    msg_type: MessageType,
    proposal_number: u64,
    proposal_value: Option<String>,
}

enum Role {
    Proposer,
    Acceptor,
    Learner,
}

struct Node {
    id: u64,
    role: Role,
    accepted_proposal: Option<Message>,
    promised_proposal_number: u64,
    proposal_number: u64,            // for Proposer
    proposal_value: Option<String>,  // for Proposer
    promises: HashSet<u64>,          // for Proposer
    proposals: HashMap<u64, String>, // for Learner
}

impl Node {
    fn new(id: u64, role: Role) -> Node {
        Node {
            id,
            role,
            accepted_proposal: None,
            promised_proposal_number: 0,
            proposal_number: 0,
            proposal_value: None,
            promises: HashSet::new(),
            proposals: HashMap::new(),
        }
    }

    fn handle_message(&mut self, message: Message) -> Option<Message> {
        match self.role {
            Role::Acceptor => self.handle_acceptor_message(message),
            Role::Proposer => self.handle_proposer_message(message),
            Role::Learner => self.handle_learner_message(message),
        }
    }

    fn handle_acceptor_message(&mut self, message: Message) -> Option<Message> {
        match message.msg_type {
            MessageType::Prepare => {
                if message.proposal_number > self.promised_proposal_number {
                    self.promised_proposal_number = message.proposal_number;
                    Some(Message {
                        msg_type: MessageType::Promise,
                        proposal_number: message.proposal_number,
                        proposal_value: message.proposal_value.clone(),
                    })
                } else {
                    None
                }
            }
            MessageType::Accept => {
                if message.proposal_number >= self.promised_proposal_number {
                    self.promised_proposal_number = message.proposal_number;
                    self.accepted_proposal = Some(message.clone());
                    Some(Message {
                        msg_type: MessageType::Accepted,
                        proposal_number: message.proposal_number,
                        proposal_value: message.proposal_value.clone(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_proposer_message(&mut self, message: Message) -> Option<Message> {
        match message.msg_type {
            MessageType::Promise => {
                self.promises.insert(message.proposal_number);
                if self.promises.len() > 1 {
                    // assume there are only 3 Acceptors
                    Some(self.send_accept())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_learner_message(&mut self, message: Message) -> Option<Message> {
        match message.msg_type {
            MessageType::Accepted => {
                if let Some(value) = message.proposal_value {
                    self.proposals.insert(message.proposal_number, value);
                }
                None
            }
            _ => None,
        }
    }

    fn send_prepare(&mut self) -> Message {
        self.proposal_number += 1;
        self.promises.clear();
        Message {
            msg_type: MessageType::Prepare,
            proposal_number: self.proposal_number,
            proposal_value: self.proposal_value.clone(),
        }
    }

    fn send_accept(&self) -> Message {
        Message {
            msg_type: MessageType::Accept,
            proposal_number: self.proposal_number,
            proposal_value: self.proposal_value.clone(),
        }
    }

    // The way Proposer set value
    fn propose(&mut self, value: String) {
        self.proposal_value = Some(value);
    }
}

fn main() {
    // 创建节点
    let mut proposer = Node::new(1, Role::Proposer);
    let mut acceptor1 = Node::new(2, Role::Acceptor);
    let mut acceptor2 = Node::new(3, Role::Acceptor);
    let mut learner = Node::new(4, Role::Learner);

    // Proposer 发起提议
    proposer.propose("Value1".to_string());

    // Proposer 发起 Prepare 请求
    let prepare_message = proposer.send_prepare();

    // Acceptor 处理 Prepare 请求
    let promise1 = acceptor1.handle_message(prepare_message.clone());
    let promise2 = acceptor2.handle_message(prepare_message);

    // Proposer 处理 Promise 消息，并发起 Accept 请求
    let accept1 = promise1.and_then(|p| proposer.handle_message(p));
    let accept2 = promise2.and_then(|p| proposer.handle_message(p));

    // Acceptor receive accept and send accepted massage
    if let Some(accept1) = accept1 {
        if let Some(accepted1) = acceptor1.handle_message(accept1) {
            println!("{}", accepted1.clone().proposal_value.unwrap());
            learner.handle_message(accepted1);
        }
    }

    if let Some(accept2) = accept2 {
        if let Some(accepted2) = acceptor2.handle_message(accept2) {
            learner.handle_message(accepted2);
        }
    }

    // 打印 Learner 学到的提议
    for (proposal_number, proposal_value) in &learner.proposals {
        println!(
            "Learner learned proposal {}: {}",
            proposal_number, proposal_value
        );
    }
}
