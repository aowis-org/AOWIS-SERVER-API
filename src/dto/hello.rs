use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Greet {
    pub name: String,
}
