use event_broker::{receive_event, send_event, Event};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Event Broker] Initializing central message hub...");

    let addr = "0.0.0.0:8082";
    let listener = TcpListener::bind(addr).await?;
    println!("[Event Broker] Server listening for services on {}", addr);

    let (tx, _rx) = broadcast::channel::<Event>(100);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        println!("[Event Broker] New service instance registered from: {}", client_addr);

        let (mut reader, mut writer) = stream.into_split();
        
        let tx_clone = tx.clone();
        let mut rx_clone = tx.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = rx_clone.recv().await {
                if let Err(e) = send_event(&mut writer, &event).await {
                    eprintln!("[Event Broker] Link breakdown writing to service stream: {}", e);
                    break;
                }
            }
        });

        tokio::spawn(async move {
            loop {
                match receive_event(&mut reader).await {
                    Ok(Some(event)) => {
                        println!("[Event Broker] Broadcast distributed for event type: {:?}", event);
                        let _ = tx_clone.send(event);
                    }
                    Ok(None) => {
                        println!("[Event Broker] Service client safely disconnected from address: {}", client_addr);
                        break;
                    }
                    Err(e) => {
                        eprintln!("[Event Broker] Network parsing fault on stream {}: {}", client_addr, e);
                        break;
                    }
                }
            }
        });
    }
}