use crate::core::types::VersionType;
use serde_json::Value;

pub async fn fetch_list(version_type: VersionType) -> Vec<Value> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let response: Value = reqwest::get(url)
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON response");
    let versions_array = response["versions"]
        .as_array()
        .expect("Failed to find 'versions' array in manifest");
    let target_type = version_type.as_str();
    let filtered_versions: Vec<Value> = versions_array
        .iter()
        .filter(|v| v["type"].as_str() == Some(target_type))
        .cloned()
        .collect();

    filtered_versions
}


