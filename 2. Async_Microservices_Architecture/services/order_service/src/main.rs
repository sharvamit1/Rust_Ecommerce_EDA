/*
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
*/
use event_broker::{receive_event, send_event, Event};
use shared_models::OrderRequest;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Order Service] Starting up in Event-Driven Mode...");

    let broker_addr = "127.0.0.1:8082";

    let mut sub_stream = TcpStream::connect(broker_addr).await?;
    tokio::spawn(async move {
        println!("[Order Service] Subscribed to Event Broker for background events...");
        loop {
            match receive_event(&mut sub_stream).await {
                Ok(Some(Event::PaymentProcessed { order_id, success, transaction_id })) => {
                    if success {
                        println!("[Order Service] 🔔 Event Received: Payment SUCCESS for Order #{} (Tx: {})", order_id, transaction_id);
                    } else {
                        println!("[Order Service] 🔔 Event Received: Payment FAILED for Order #{}", order_id);
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => {
                    println!("[Order Service] Broker background channel closed.");
                    break;
                }
                Err(e) => {
                    eprintln!("[Order Service] Error reading from background broker stream: {}", e);
                    break;
                }
            }
        }
    });

    let pub_stream = TcpStream::connect(broker_addr).await?;
    let shared_pub_stream = Arc::new(Mutex::new(pub_stream));

    let client_listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("[Order Service] Ingestion API listening for clients on port 8080");

    let mut mock_order_id_counter = 1000;

    loop {
        let (mut client_socket, _) = client_listener.accept().await?;
        let pub_stream_clone = Arc::clone(&shared_pub_stream);
        mock_order_id_counter += 1;
        let current_order_id = mock_order_id_counter;

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            match client_socket.read(&mut buffer).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    if let Ok(req) = serde_json::from_slice::<OrderRequest>(&buffer[..bytes_read]) {
                        println!("[Order Service] 📥 Client Request received for item #{}", req.item_id);

                        let order_event = Event::OrderCreated {
                            order_id: current_order_id,
                            item_id: req.item_id,
                            quantity: req.quantity,
                            user_id: req.user_id,
                        };

                        let mut broker_lock = pub_stream_clone.lock().await;
                        if let Err(e) = send_event(&mut *broker_lock, &order_event).await {
                            eprintln!("[Order Service] Failed to dispatch event to broker: {}", e);
                            let _ = client_socket.write_all(b"Internal server error").await;
                            return;
                        }
                        drop(broker_lock);

                        let client_response = format!("Order accepted. Order ID: {}", current_order_id);
                        if let Err(e) = client_socket.write_all(client_response.as_bytes()).await {
                            eprintln!("[Order Service] Failed to acknowledge client: {}", e);
                        }
                    } else {
                        let _ = client_socket.write_all(b"Invalid order request payload").await;
                    }
                }
                _ => {}
            }
        });
    }
}
/*
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjuramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
ramramjiramramjiramramji
*/