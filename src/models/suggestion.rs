use crate::models::product::Product;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub title: String,
    pub body: String,
    pub learning_url: Vec<String>,
    pub working_url: Vec<String>,
    pub products: Vec<serde_json::map::Map<String, serde_json::value::Value>>,
}
