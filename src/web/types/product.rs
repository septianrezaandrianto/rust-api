use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::domain::product::Product;

#[derive(Debug, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub category: String,
    pub qty: i32,
    pub price: Decimal,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub qty: Option<i32>,
    pub price: Decimal,
    pub is_deleted: bool,
    pub created_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            name: p.name,
            category: p.category,
            qty: p.qty,
            price: p.price,
            is_deleted: p.is_deleted,
            created_date: p.created_date,
            modified_date: p.modified_date,
        }
    }
}
