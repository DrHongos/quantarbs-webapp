use serde_json::Value;
use std::collections::HashMap;

pub async fn fetch_tokens() -> Vec<String> {
    let data = reqwest::get("http://opinologos.xyz:8080/get_tokens")
        .await
        .expect("Error calling api: tokens")
        .json::<Vec<String>>()
        .await
        .expect("Error parsing: tokens");
    data
}


// get opentime & close price of each token
pub async fn fetch_token_data(start: String, end: String, name: String) -> Vec<Value> {
        let mut map = HashMap::new();
        map.insert("start", &start);
        map.insert("end", &end);
        map.insert("name", &name);
        // 
        let client = reqwest::Client::new();

        let res: Vec<Value> = client.post("http://opinologos.xyz:8080/get_range_close")
            .json(&map)
            .send()
            .await
            .expect("Could not send post: get token data")
            .json::<Vec<Value>>()
            .await
            .expect("Server did not return token data");
        //logging::log!("Received {:#?}", res);
        res
}

pub fn simplify(d: String) -> String {
    format!("{} 00:00:00", d)           // parsed for server query
}