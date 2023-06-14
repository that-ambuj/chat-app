use actix_ws::{CloseReason, Message, MessageStream, ProtocolError, Session};
use futures_util::StreamExt;
use std::time::{Duration, Instant};
use tokio::{pin, select, time::interval};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn echo_ws(mut session: Session, mut msg_stream: MessageStream) {
    tracing::info!("Connected to Websocket");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // create "next client timeout check" future
        let tick = interval.tick();
        // required for select()
        pin!(tick);

        // waits for either `msg_stream` to receive a message from the client or the heartbeat
        // interval timer to tick, yielding the value of whichever one is ready first
        select! {
            // received message from WebSocket client
            Some(message) = msg_stream.next() => {
                tracing::info!("msg: {message:?}");

                if let Err(reason) = handle_message(message, &mut session, &mut last_heartbeat).await {
                    break reason;
                }
            },

            // heartbeat interval ticked
            _inst = tick => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    tracing::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }


        }
    };

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    tracing::info!("disconnected");
}

async fn handle_message(
    message: Result<Message, ProtocolError>,
    session: &mut Session,
    last_heartbeat: &mut Instant,
) -> Result<(), Option<CloseReason>> {
    match message {
        Err(err) => {
            tracing::error!("{:?}", err);
            Err(None)
        }

        Ok(msg) => {
            match msg {
                Message::Text(text) => {
                    session.text(text).await.unwrap();
                    Ok(())
                }

                Message::Binary(bin) => {
                    session.binary(bin).await.unwrap();
                    Ok(())
                }

                Message::Close(reason) => {
                    return Err(reason);
                }

                Message::Ping(bytes) => {
                    *last_heartbeat = Instant::now();
                    let _ = session.pong(&bytes).await;

                    Ok(())
                }

                Message::Pong(_) => {
                    *last_heartbeat = Instant::now();

                    Ok(())
                }

                Message::Continuation(_) => {
                    tracing::warn!("no support for continuation frames");

                    Ok(())
                }

                // no-op; ignore
                Message::Nop => Ok(()),
            }
        }
    }
}
