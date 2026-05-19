use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub async fn handle_client(
    stream: TcpStream,
    tap_addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws = accept_async(stream).await?;
    let tap = TcpStream::connect(&tap_addr).await?;

    let (mut ws_write, mut ws_read) = ws.split();
    let (tap_read, mut tap_write) = tap.into_split();

    let mut tap_lines = BufReader::new(tap_read).lines();

    let ws_to_tap = async {
        while let Some(msg) = ws_read.next().await {
            match msg? {
                Message::Text(text) => {
                    let text = text.to_string();
                    let line = if text.ends_with('\n') {
                        text
                    } else {
                        format!("{}\n", text)
                    };
                    tap_write.write_all(line.as_bytes()).await?;
                    tap_write.flush().await?;
                }
                Message::Binary(bin) => {
                    tap_write.write_all(&bin).await?;
                    tap_write.flush().await?;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    let tap_to_ws = async {
        while let Some(line) = tap_lines.next_line().await? {
            let msg = Message::Text(format!("{}\n", line).into());
            ws_write.send(msg).await?;
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    tokio::select! {
        result = ws_to_tap => result?,
        result = tap_to_ws => result?,
    }

    Ok(())
}
