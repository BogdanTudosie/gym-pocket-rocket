use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::from_str;
use std::fs;


#[derive(Serialize, Deserialize)]
pub struct Gym {
    name: String,
    city: String,
    is_available: bool
}

pub fn load_gyms_from_file(file_path: &str) -> Vec<Gym> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    from_str(&data).expect("Unable to parse JSON")
}

pub fn save_gyms_to_file(file_path: &str, gyms: &Vec<Gym>) {
    let data = serde_json::to_string(gyms).expect("Failed to serialize gyms");
    fs::write(file_path, data).expect("Unable to write file");
}