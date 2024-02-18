use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: i32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl Event {
    pub fn find_first_e_tag(&self) -> Option<&String> {
        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item's first character is "e"
                if first_item.starts_with("e") {
                    // If true, return the second item if it exists
                    return inner_vec.get(1);
                }
            }
        }
        // If no match is found, return None
        None
    }

    pub fn find_second_e_tag(&self) -> Option<&String> {
        // Counter to track the number of matching occurrences
        let mut count = 0;

        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item's first character is "e"
                if first_item.starts_with("e") {
                    // Increment the counter
                    count += 1;

                    // If it's the second occurrence, return the string after it
                    if count == 2 {
                        return inner_vec.get(1);
                    }
                }
            }
        }
        // If no second match is found, return None
        None
    }

    pub fn find_first_tag(&self, input: &str) -> Option<Vec<String>> {
        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item starts with the input string
                if first_item.starts_with(input) {
                    // Clone the inner vector and remove the input string
                    let result = inner_vec.iter().skip(1).cloned().collect();
                    return Some(result);
                }
            }
        }
        // If no match is found, return None
        None
    }
}

pub const NOSCRIPT_KIND: i32 = 32043;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoscriptPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture: Option<String>,
    pub version: Option<String>,
    pub source_code: Option<String>, // source code url
    pub published_at: Option<i64>,   // timestamp, seconds
    pub runtime_version: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoscriptContent {
    pub wasm: String,
    pub binding: String,
}

impl ToString for NoscriptContent {
    fn to_string(&self) -> String {
        let serialized: String = serde_json::to_string(&self).unwrap();
        serialized
    }
}
