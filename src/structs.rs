use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ClosePrices {
    pub opentime: String,
    pub close: f64
}

impl From<Value> for ClosePrices {
    fn from(value: Value) -> Self {
        ClosePrices { 
            opentime: value["opentime"].as_str().unwrap().to_owned(), 
            close: value["close"].as_f64().unwrap() 
        }
    }   
}

    