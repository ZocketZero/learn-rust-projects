use serde::Deserialize;

#[derive(serde::Serialize, Deserialize)]
pub struct Product {
    id: u32,
    pub name: String,
    pub price: u32,
}

impl Product {
    pub fn new(id: u32, name: &str, price: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
        }
    }
}

#[derive(serde::Serialize, Deserialize)]
pub struct ProductInput {
    pub name: String,
    pub price: u32,
}

impl ProductInput {
    pub fn new(name: &str, price: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}

pub type ProductList = Vec<Product>;
