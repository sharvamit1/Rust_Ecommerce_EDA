use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use shared_models::{OrderRequest, PaymentRequest, PaymentResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Order service running on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            match socket.read(&mut buffer).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    if let Ok(order_req) = serde_json::from_slice::<OrderRequest>(&buffer[..bytes_read]) {
                        println!("Processing order for item: {}, quantity: {}", order_req.item_id, order_req.quantity);

                        let amount = (order_req.quantity as f64) * 25.0;
                        let mock_order_id = 12345;

                        let payment_req = PaymentRequest {
                            order_id: mock_order_id,
                            amount,
                            user_id: order_req.user_id,
                        };

                        match TcpStream::connect("127.0.0.1:8081").await {
                            Ok(mut payment_stream) => {
                                if let Ok(serialized_payment) = serde_json::to_vec(&payment_req) {
                                    if payment_stream.write_all(&serialized_payment).await.is_ok() {

                                        let _ = payment_stream.shutdown().await;

                                        let mut payment_buffer = [0; 1024];
                                        match payment_stream.read(&mut payment_buffer).await {
                                            Ok(p_bytes) if p_bytes > 0 => {
                                                if let Ok(payment_res) = serde_json::from_slice::<PaymentResponse>(&payment_buffer[..p_bytes]) {
                                                    if payment_res.success {
                                                        println!("Order successfully completed with Tx: {}", payment_res.transaction_id);
                                                        let _ = socket.write_all(b"Order completed successfully").await;
                                                    } else {
                                                        println!("Order failed due to payment rejection");
                                                        let _ = socket.write_all(b"Order failed: Payment rejected").await;
                                                    }
                                                }
                                            }
                                            _ => {
                                                let _ = socket.write_all(b"Order failed: Invalid payment response").await;
                                            }
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                println!("Failed to connect to Payment Service");
                                let _ = socket.write_all(b"Order failed: Payment service unavailable").await;
                            }
                        }
                    }
                }
                _ => {}
            }
        });
    }
}