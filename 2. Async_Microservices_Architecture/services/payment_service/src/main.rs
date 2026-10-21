use event_broker::{receive_event, send_event, Event};
use std::time::Duration;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Payment Service] Starting up in Event-Driven Mode...");
    
    let broker_addr = "127.0.0.1:8082";
    let mut stream = TcpStream::connect(broker_addr).await?;
    println!("[Payment Service] Connected to Event Broker at {}", broker_addr);

    loop {
        match receive_event(&mut stream).await {
            Ok(Some(event)) => {
                match event {
                    Event::OrderCreated { order_id, item_id, quantity, user_id } => {
                        println!(
                            "[Payment Service] 📥 Processing OrderCreated event: Order #{}, Item #{}, Qty: {}, User: {}",
                            order_id, item_id, quantity, user_id
                        );

                        tokio::time::sleep(Duration::from_millis(500)).await;

                        let success = user_id != 999;
                        let transaction_id = if success {
                            format!("tx_eda_{}", order_id)
                        } else {
                            "".to_string()
                        };

                        if success {
                            println!("[Payment Service] ✅ Payment successful for Order #{}", order_id);
                        } else {
                            println!("[Payment Service] ❌ Payment rejected for Order #{} due to insufficient funds", order_id);
                        }

                        let response_event = Event::PaymentProcessed {
                            order_id,
                            success,
                            transaction_id,
                        };

                        if let Err(e) = send_event(&mut stream, &response_event).await {
                            eprintln!("[Payment Service] Failed to send PaymentProcessed event: {}", e);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            Ok(None) => {
                println!("[Payment Service] Connection closed by Event Broker.");
                break;
            }
            Err(e) => {
                eprintln!("[Payment Service] Error receiving event: {}", e);
                break;
            }
        }
    }

    Ok(())
}