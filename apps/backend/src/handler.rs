use actix_ws::{CloseReason, Message as WsMessage, MessageStream, Session};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::{select, sync::mpsc, time::interval};

use crate::AppState;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn chat_ws(mut ctx: Session, mut msg_stream: MessageStream, state: Arc<AppState>) {
    tracing::info!("Connected to Websocket");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let new_id = fastrand::usize(..9999);
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    {
        let mut sessions = state.sessions.lock().unwrap();
        sessions.insert(new_id, msg_tx);
        dbg!(&sessions.len());
    }

    tracing::info!("User {new_id} connected.");

    let reason = loop {
        // create "next client timeout check" future
        // let tick = interval.tick();
        // let ws_messages = msg_stream.next();

        select! {
            Some(message) = msg_rx.recv() => {
                last_heartbeat = Instant::now();
                ctx.text(message).await.unwrap();
            }

            Some(Ok(ws_msg)) = msg_stream.next() => {
                tracing::debug!("Recieved ws_msg: {ws_msg:?}");

                if let Err(reason) = process_ws_msg(ws_msg, &mut last_heartbeat, &mut ctx, new_id, state.clone()).await {
                    break reason;
                }
            }

            _ = interval.tick() => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    tracing::warn!("Client has not sent any heartbeats for more that {CLIENT_TIMEOUT:?}; disconnecting.");
                    break None;
                }

                // send heartbeat ping
                let _ = ctx.ping(b"").await;
            }
        }
    };

    // attempt to close connection gracefully
    let _ = ctx.close(reason).await;

    tracing::info!("User {new_id:?} disconnected");
}

async fn process_ws_msg(
    msg: WsMessage,
    last_heartbeat: &mut Instant,
    ctx: &mut Session,
    skip_id: usize,
    state: Arc<AppState>,
) -> Result<(), Option<CloseReason>> {
    use WsMessage::*;

    match msg {
        Text(txt) => {
            for (id, msg_tx) in state.sessions.lock().unwrap().iter() {
                if *id != skip_id {
                    msg_tx.send(txt.clone().into()).unwrap();
                }
            }

            Ok(())
        }
        Binary(_bin) => {
            tracing::warn!("Did not expect binary data.");

            Err(None)
        }

        Close(reason) => Err(reason),

        Ping(bytes) => {
            *last_heartbeat = Instant::now();
            ctx.pong(&bytes).await.unwrap();

            Ok(())
        }

        Pong(_) => {
            *last_heartbeat = Instant::now();

            Ok(())
        }

        Continuation(_) => {
            tracing::warn!("no support for continuation frames");

            Err(None)
        }

        // ignore
        _ => Ok(()),
    }
}
