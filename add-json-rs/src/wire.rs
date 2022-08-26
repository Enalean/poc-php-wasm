use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonData {
    pub number1: u64,
    pub number2: u64,
    pub res: u64
}