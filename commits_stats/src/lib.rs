use std::collections::HashMap;
use json;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();
    
    if let json::JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(week) = commit["week"].as_str() {
                *week_counts.entry(week.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    week_counts
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();
    
    if let json::JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(author) = commit["author"].as_str() {
                *author_counts.entry(author.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    author_counts
}