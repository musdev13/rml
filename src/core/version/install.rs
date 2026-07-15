use std::path::PathBuf;

use crate::core::config;

pub fn install(version_id: String, directory: Option<PathBuf>){
   println!("{}", musutils::fs::tilda_desir(config::get_versions_path()).display()); 


    // let config = musutils::fs::config::get("rml", "paths.json", Some(&serde_json::to_string_pretty(&*DEFAULT_PATHS_CONFIG).unwrap()));
    // let Some(versions_path) = musutils::fs::config::get_value(&serde_json::from_str(&config).expect("can't serialize"), "versions_path") else {
    //     panic!("errar");
    // };
    // println!("{}", versions_path);
}
