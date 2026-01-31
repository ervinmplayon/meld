use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use chrono::{DateTime, Utc};

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
struct NonEksRow {
    #[serde(rename = "Repo Name")]
    repo_name: String,
    #[serde(rename = "Platform")]
    platform: String,
    #[serde(rename = "CI/CD Platform")]
    cicd: String,
    #[serde(rename = "Has Tests")]
    has_tests: String,
    #[serde(rename = "Test Framework")]
    test_setup: String,
    #[serde(rename = "Repo URL")]
    repo_url: String,
    #[serde(rename = "Is Archived")]
    is_archived: String,

    // Backups for last committer and last commit date
    #[serde(rename = "Last Committer")]
    last_committer: String,
    #[serde(rename = "Last Commit Date")]
    last_commit_date: String,
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
    test_setup: String,
    repo_url: String,
    is_archived: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let base_path = home::home_dir().unwrap().join("Documents/csv/github");
    let owners_path = base_path.join("repo_owners.csv");
    let non_eks_path = base_path.join("non_eks.csv");
    let output_path = base_path.join("ghub_consolidated.csv");

    // Load repo_owners into a HashMap with De-duplication logic
    let mut rdr = csv::Reader::from_path(owners_path)?;
    let mut owners_map: HashMap<String, RepoOwnerRow> = HashMap:: new();

    for result in rdr.deserialize() {
        let record: RepoOwnerRow = result?;
        let key = record.repo.to_lowercase();

        // Parse ISO 8601: 2025-10-15T14:37:19Z
        let current_dt = record.last_commit_date.parse::<DateTime<Utc>>().unwrap_or(Utc::now());

        // Returns Some(&value), unwraps that value and binds it to a new local variable
        if let Some(existing) = owners_map.get(&key) {
            // Parse dates to compare which is more recent
            let existing_dt = existing.last_commit_date.parse::<DateTime<Utc>>().unwrap_or(Utc::now());

            if current_dt > existing_dt {
                owners_map.insert(key, record);
            }
        } else {
            owners_map.insert(key, record);
        }
    }

    // Non-EKS (The primary driver for the Left Join)
    let mut rdr = csv::Reader::from_path(non_eks_path)?;
    let mut wtr = csv::Writer::from_path(output_path)?;

    for result in rdr.deserialize() {
        let non_eks: NonEksRow = result?;
        let lookup_key = non_eks.repo_name.to_lowercase();
        let owner = owners_map.get(&lookup_key);

        let final_record = FinalRow {
            repo: owner.map(|o| o.repo.clone()).unwrap_or(non_eks.repo_name),
            last_committer: owner.map(|o| o.last_committer.clone()).unwrap_or(non_eks.last_committer),
            last_commit_date: owner.map(|o| o.last_commit_date.clone()).unwrap_or(non_eks.last_commit_date),
            updated_at: owner.map(|o| o.updated_at.clone()).unwrap_or_default(),
            primary_owner: owner.map(|o| o.primary_owner.clone()).unwrap_or_default(),
            team_owners_admin_maintain: owner.map(|o| o.team_owners_admin_maintain.clone()).unwrap_or_default(),
            visibility: owner.map(|o| o.visibility.clone()).unwrap_or_default(),
            platform: non_eks.platform,
            cicd: non_eks.cicd,
            has_tests: non_eks.has_tests,
            test_setup: non_eks.test_setup,
            repo_url: non_eks.repo_url,
            is_archived: non_eks.is_archived,
        };
        wtr.serialize(final_record)?;
    }

    wtr.flush()?;
    println!("Consolidation complete. Saved to: ~/Documents/csv/github/ghub_consolidated.csv");
    Ok(())
}
