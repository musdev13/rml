use crate::core::{types::VersionType, version::{fetch_list, strip_version}};

pub async fn handle(
    version_type: VersionType, 
    page: u32, 
    count: u32, 
    json: bool,
    id: bool,
    release_time: bool,
    time: bool,
    r#type: bool,
    url: bool,
) {
    let versions = fetch_list(version_type).await;
    let striped_versions = strip_version(
        &musutils::vec::get_page(&versions, page, count), 
        id, 
        release_time, 
        time, 
        r#type, 
        url
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&striped_versions).expect("can't serialize"));
    } else {
        if striped_versions.is_empty() {
            println!("no versions available on page {}.", page);
            return;
        }

        println!("\n--- versions list (page {}, items: {}) ---", page, striped_versions.len());

        let keys: Vec<String> = match striped_versions[0].as_object() {
            Some(obj) => obj.keys().cloned().collect(),
            None => vec![],
        };

        if keys.is_empty() {
            println!("no fields to display");
            return;
        }

        let header = keys
            .iter()
            .map(|k| format!("{:<25}", k))
            .collect::<Vec<String>>()
            .join(" | ");
        
        let line_len = header.len();
        println!("{}", header);
        println!("{}", "-".repeat(line_len));

        for v in &striped_versions {
            if let Some(obj) = v.as_object() {
                let row = keys
                    .iter()
                    .map(|k| {
                        let val_str = match obj.get(k) {
                            Some(serde_json::Value::String(s)) => s.clone(),
                            Some(other) => other.to_string(),
                            None => "N/A".to_string(),
                        };
                        format!("{:<25}", val_str)
                    })
                    .collect::<Vec<String>>()
                    .join(" | ");
                
                println!("{}", row);
            }
        }
        println!("{}\n", "-".repeat(line_len));
    }
}
