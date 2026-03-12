use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

pub type ApiKeys = Arc<HashMap<String, f64>>;

pub fn load_api_keys() -> ApiKeys {
    let data =
        fs::read_to_string("api_keys.json").expect("failed to read api_keys.json");

    let api_keys_map: HashMap<String, f64> =
        serde_json::from_str(&data).expect("invalid json");

    Arc::new(api_keys_map)
}