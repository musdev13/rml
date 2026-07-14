use crate::core::config::DEFAULT_PATHS_CONFIG;

pub fn get_versions_path() -> String {
    let config_str = musutils::fs::config::get("rml", "paths.json", Some(&serde_json::to_string_pretty(&*DEFAULT_PATHS_CONFIG).unwrap()));
    let versions_path = musutils::fs::config::get_value(&config_json, "versions_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .expect("there is no versions_path or it is not a string");

    versions_path
}
