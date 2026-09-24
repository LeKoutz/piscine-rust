use std::{collections::HashMap};

use chrono::prelude::*;

struct CommitData {
    author: String,
    date: DateTime<Utc>,
}

fn parse_json(data: &json::JsonValue) -> Vec<CommitData> {
    let mut commits: Vec<CommitData> = Vec::new();
    for commit in data.members() {
        let author = commit["author"]["login"].as_str().unwrap().to_string();
        let date = commit["commit"]["author"]["date"].as_str().unwrap().to_string();
        let commit_data = CommitData {
            author,
            date: DateTime::parse_from_rfc3339(&date).unwrap().into(),
        };
        commits.push(commit_data);
    }
    commits
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for commit in parse_json(data) {
        let week = commit.date.iso_week();
        let key = format!("{}-W{}", week.year(), week.week());
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new(); 
    for commit in parse_json(data) {
        *counts.entry(commit.author).or_insert(0) += 1;
    }
	counts
}
