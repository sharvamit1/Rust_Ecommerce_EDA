use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use shared_models::{PaymentRequest, PaymentResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Payment service running on 127.0.0.1:8081");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            
            match socket.read(&mut buffer).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    if let Ok(req) = serde_json::from_slice::<PaymentRequest>(&buffer[..bytes_read]) {
                        println!("Received payment request for order: {}, user: {}", req.order_id, req.user_id);
                        
                        let success = req.user_id != 999;
                        let transaction_id = if success {
                            format!("tx_{}", req.order_id)
                        } else {
                            String::new()
                        };

                        let res = PaymentResponse { success, transaction_id };
                        
                        if let Ok(serialized) = serde_json::to_vec(&res) {
                            let _ = socket.write_all(&serialized).await;
                        }
                    }
                }
                _ => {}
            }
        });
    }
}