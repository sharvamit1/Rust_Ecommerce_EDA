use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrderRequest {
    pub item_id: u32,
    pub quantity: u32,
    pub user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentRequest {
    pub order_id: u64,
    pub amount: f64,
    pub user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentResponse {
    pub success: bool,
    pub transaction_id: String,
}

impl OrderRequest {
    pub fn new(item_id: u32, quantity: u32, user_id: u32) -> Self {
        Self {
            item_id,
            quantity,
            user_id,
        }
    }
}

impl PaymentRequest {
    pub fn new(order_id: u64, amount: f64, user_id: u32) -> Self {
        Self {
            order_id,
            amount,
            user_id,
        }
    }
}