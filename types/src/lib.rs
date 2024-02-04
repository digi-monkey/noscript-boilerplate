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

pub const NOSCRIPT_KIND: i32 = 32042;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoscriptPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture: Option<String>,
    pub version: Option<String>,
    pub source_codeversion: Option<String>,
    pub source_code: Option<String>, // source code url
    pub published_at: Option<i64>,   // timestamp, seconds
}

pub enum FilterOptMode {
    global = 0,
    follow = 1,
    trust_network = 2, // follow's follow without spam
    sign_in_user = 3,
    visiting_user = 4,
    custom = 5,
}

impl FilterOptMode {
    pub fn to_string(&self) -> String {
        match *self {
            FilterOptMode::global => "0".to_string(),
            FilterOptMode::follow => "1".to_string(),
            FilterOptMode::trust_network => "2".to_string(),
            FilterOptMode::sign_in_user => "3".to_string(),
            FilterOptMode::visiting_user => "4".to_string(),
            FilterOptMode::custom => "5".to_string(),
        }
    }
}
