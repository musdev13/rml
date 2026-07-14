use serde_json::Value;

pub async fn fetch_list() -> Vec<Value> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let response: Value = reqwest::get(url)
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON response");

    response["versions"]
        .as_array()
        .expect("Failed to find 'versions' array in manifest")
        .clone()
}


