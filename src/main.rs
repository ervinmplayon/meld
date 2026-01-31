use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Clone)]
struct RepoOwnerRow {
    repo: String,
    last_committer: String,
    last_commit_date: String, // for raw storage, keep as string
    updated_at: String,
    primary_owner: String,
    team_owners_admin_maintain: String,
    visibility: String,
}

#[derive(Debug, Deserialize)]
struct NonEksROw {
    #[serde(rename = "Repo Name")]
    repo_name: String,
    #[serde(rename = "Platform")]
    platform: String,
    #[serde(rename = "CI/CD")]
    cicd: String,
    #[serde(rename = "Has Tests")]
    has_tests: String,
    #[serde(rename = "Test Framework")]
    test_framework: String,
    #[serde(rename = "Repo URL")]
    repo_url: String,
    #[serde(rename = "Is Archived")]
    is_archived: String,
}

#[derive(Serialize, Default)]
struct FinalRow {
    repo: String,
    last_committer: String,
    last_commit_date: String,
    updated_at: String,
    primary_owner: String,
    team_owners_admin_maintain: String,
    visibility: String,
    platform: String,
    cicd: String,
    has_tests: String,
    test_framework: String,
    repo_url: String,
    is_archived: String,
}

fn main() {
    println!("Hello, world!");
}
