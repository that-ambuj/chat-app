use actix_ws::{CloseReason, Message as WsMessage, MessageStream, Session};
use futures_util::StreamExt;
use std::time::{Duration, Instant};
use tokio::{select, sync::mpsc, time::interval};

use crate::chat_server::{ChatServerHandle, ConnId};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(serde::Deserialize)]
struct ChatMessage {
    message: String,
    to: Option<ConnId>,
}

pub async fn chat_ws(
    mut ctx: Session,
    mut msg_stream: MessageStream,
    cmd_handle: ChatServerHandle,
) {
    tracing::info!("Connected to Websocket");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    let conn_id = cmd_handle.connect(conn_tx).await;

    tracing::debug!("User {conn_id} connected.");

    let reason = loop {
        select! {
            biased;
            _ = interval.tick() => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    tracing::warn!("Client has not sent any heartbeats for more that {CLIENT_TIMEOUT:?}; disconnecting.");
                    break None;
                }

                // send heartbeat ping
                let _ = ctx.ping(b"").await;
            }

            Some(message) = conn_rx.recv() => {
                last_heartbeat = Instant::now();
                ctx.text(message).await.unwrap();
            }

            Some(Ok(ws_msg)) = msg_stream.next() => {
                tracing::debug!("Recieved ws_msg: {ws_msg:?}");

                if let Err(reason) = process_ws_msg(
                    ws_msg,
                    &mut last_heartbeat,
                    &mut ctx,
                    conn_id,
                    cmd_handle.clone())
                    .await
                {
                    break reason;
                }
            }
        }
    };

    // attempt to close connection gracefully
    let _ = ctx.close(reason).await;
    cmd_handle.disconnect(conn_id).await;

    tracing::debug!("User {conn_id} disconnected");
}

async fn process_ws_msg(
    msg: WsMessage,
    last_heartbeat: &mut Instant,
    ctx: &mut Session,
    from: ConnId,
    cmd_handle: ChatServerHandle,
) -> Result<(), Option<CloseReason>> {
    use WsMessage::*;

    match msg {
        Text(txt) => {
            let chat_message = serde_json::from_str::<ChatMessage>(&txt.to_string()).unwrap();

            if chat_message.message == "/list" {
                let users = cmd_handle.list_users().await;
                let _ = ctx.text(serde_json::to_string(&users).unwrap()).await;

                return Ok(());
            }

            cmd_handle
                .send_message(&chat_message.message, from, chat_message.to)
                .await;

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
