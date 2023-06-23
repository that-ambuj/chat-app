use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, oneshot};

pub type ConnId = usize;
// pub type Message = String;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Message {
    pub message: String,
    pub to: Option<ConnId>,
    pub from: ConnId,
}

pub struct ChatServer {
    sessions: HashMap<ConnId, mpsc::UnboundedSender<Message>>,
    visitor_count: Arc<AtomicUsize>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

#[derive(Clone, Debug)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Message>,
        res_tx: oneshot::Sender<ConnId>,
    },
    Disconnect {
        conn: ConnId,
    },
    Message {
        message: Message,
        res_tx: oneshot::Sender<()>,
    },
    List {
        res_tx: oneshot::Sender<Vec<ConnId>>,
    },
}

impl ChatServer {
    pub fn new() -> (Self, ChatServerHandle) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            ChatServer {
                sessions: HashMap::new(),
                visitor_count: Arc::new(AtomicUsize::new(1)),
                cmd_rx,
            },
            ChatServerHandle { cmd_tx },
        )
    }

    async fn broadcast_message(&self, from: ConnId, message: &str) {
        let msg = Message {
            message: message.into(),
            from,
            to: None,
        };

        for (_, msg_tx) in self.sessions.iter() {
            let _ = msg_tx.send(msg.clone());
        }
    }

    async fn send_message(&self, msg: &Message) {
        let Message {
            message: _,
            to,
            from,
        } = msg;

        if let Some(to) = to {
            if let Some(msg_tx) = self.sessions.get(&to) {
                let _ = msg_tx.send(msg.clone());
            }
        } else {
            self.broadcast_message(*from, &msg.message).await;
        }
    }

    async fn connect(&mut self, conn_tx: mpsc::UnboundedSender<Message>) -> usize {
        // Let's keep the user_id number 4 digits for readability
        let id = fastrand::usize(..10_000);
        self.sessions.insert(id, conn_tx);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.broadcast_message(
            id,
            &format!("User {id} just connected.(Visitor count: {count})"),
        )
        .await;

        id
    }

    fn list_all(&self) -> Vec<ConnId> {
        self.sessions.keys().map(|k| *k).collect()
    }

    async fn disconnect(&mut self, conn: ConnId) {
        // Remove the connection regardless it existed in the first place
        if self.sessions.remove(&conn).is_some() {
            let count = self.visitor_count.fetch_sub(1, Ordering::SeqCst);

            self.broadcast_message(
                conn,
                &format!("User {conn} just disconnected.(Visitor count: {count})"),
            )
            .await;
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        use Command::*;

        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }
                Disconnect { conn } => {
                    self.disconnect(conn).await;
                }
                Message { message, res_tx } => {
                    self.send_message(&message).await;
                    let _ = res_tx.send(());
                }
                List { res_tx } => {
                    let list = self.list_all();
                    let _ = res_tx.send(list);
                }
            }
        }

        Ok(())
    }
}

impl ChatServerHandle {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Message>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::Connect { conn_tx, res_tx })
            .unwrap();

        res_rx.await.unwrap()
    }

    pub async fn disconnect(&self, conn: ConnId) {
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }

    pub async fn list_users(&self) -> Vec<ConnId> {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::List { res_tx }).unwrap();

        res_rx.await.unwrap()
    }

    pub async fn send_message(&self, message: &str, from: ConnId, to: Option<ConnId>) {
        let (res_tx, res_rx) = oneshot::channel();

        let msg = Message {
            message: message.into(),
            from,
            to,
        };

        self.cmd_tx
            .send(Command::Message {
                message: msg,
                res_tx,
            })
            .unwrap();

        res_rx.await.unwrap();
    }
}
