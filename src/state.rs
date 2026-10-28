use crate::models::Product;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct AppState {
    pub products: Arc<Mutex<Vec<Product>>>,
}

impl AppState {
    pub fn from_json(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).unwrap();
        let products: Vec<Product> = serde_json::from_str(&data).unwrap();
        Self {
            products: Arc::new(Mutex::new(products)),
        }
    }
}
