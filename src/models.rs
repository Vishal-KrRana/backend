use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub price: f32,
}
