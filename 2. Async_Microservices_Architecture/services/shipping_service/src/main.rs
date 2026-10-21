use event_broker::{receive_event, Event};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Shipping Service] Starting up in Event-Driven Mode...");

    let broker_addr = "127.0.0.1:8082";
    let mut stream = TcpStream::connect(broker_addr).await?;
    println!("[Shipping Service] Connected to Event Broker at {}", broker_addr);

    loop {
        match receive_event(&mut stream).await {
            Ok(Some(event)) => {
                match event {
                    Event::PaymentProcessed { order_id, success, transaction_id } => {
                        if success {
                            println!(
                                "[Shipping Service] 📦 Payment CONFIRMED for Order #{}. Generating shipping label and dispatching package to the courier! (Tx: {})",
                                order_id, transaction_id
                            );
                        } else {
                            println!(
                                "[Shipping Service] 🛑 Order #{} delivery setup aborted. Downstream payment check failed.",
                                order_id
                            );
                        }
                    }
                    Event::OrderCreated { .. } => {
                    }
                }
            }
            Ok(None) => {
                println!("[Shipping Service] Broker stream disconnected safely. Exiting...");
                break;
            }
            Err(e) => {
                eprintln!("[Shipping Service] Network parsing fault: {}", e);
                break;
            }
        }
    }

    Ok(())
}