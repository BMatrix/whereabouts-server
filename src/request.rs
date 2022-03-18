// Quicktype.io
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde_derive::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "lat")]
    pub lat: Option<f64>,

    #[serde(rename = "lon")]
    pub lon: Option<f64>,

    #[serde(rename = "time")]
    pub time: Option<String>,
}
