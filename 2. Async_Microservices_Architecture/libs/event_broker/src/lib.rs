use serde::{Deserialize, Serialize};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Event {
    OrderCreated {
        order_id: u64,
        item_id: u32,
        quantity: u32,
        user_id: u32,
    },
    PaymentProcessed {
        order_id: u64,
        success: bool,
        transaction_id: String,
    },
}

pub async fn send_event<W>(stream: &mut W, event: &Event) -> io::Result<()>
where
    W: AsyncWriteExt + Unpin,
{
    let serialized = serde_json::to_vec(event)?;
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&serialized).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn receive_event<R>(stream: &mut R) -> io::Result<Option<Event>>
where
    R: AsyncReadExt + Unpin,
{
    let mut len_bytes = [0u8; 4];
    match stream.read_exact(&mut len_bytes).await {
        Ok(_) => {
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut buffer = vec![0u8; len];
            stream.read_exact(&mut buffer).await?;
            let event = serde_json::from_slice(&buffer)?;
            Ok(Some(event))
        }
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}