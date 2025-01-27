use super::util::sort_vers;
use crate::db::{ivec_to_string, Database};
use crate::scrape::Scraper;
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum SortBy {
    TimeNew,
    TimeOld,
    NameNew,
    NameOld,
}

pub(super) async fn handler(
    pattern: Option<String>,
    load_order: bool,
    display_time: bool,
    sort_by: Option<SortBy>,
) {
    let db = Database::new();
    let ver_db = db.get_version_db();
    let cache_db = db.get_cache_db();
    if load_order {
        let scrap = Scraper::new().await;
        let all_opt_vers: Vec<&String> = scrap.get_all_opt_vers().iter().collect();
        let all_opt_vers_serial = bincode::serialize(&all_opt_vers).unwrap();
        cache_db
            .insert("all_opt_ver", all_opt_vers_serial)
            .expect("Failed to insert Optifine version into database");
    }
    let vers_unsorted: Vec<(String, String)> = ver_db
        .iter()
        .filter_map(|e| match e {
            Ok((k, v)) => Some((ivec_to_string(&k), ivec_to_string(&v))),
            Err(_) => None,
        })
        .filter(|(k, _)| k.contains(pattern.as_deref().unwrap_or("")))
        .collect();
    let vers = sort_vers(vers_unsorted, &cache_db, sort_by.unwrap_or(SortBy::NameNew));
    if display_time {
        let max_key_len = vers.iter().map(|e| e.0.len()).max().unwrap_or_default();
        for (k, v) in vers.iter() {
            println!("   {} {} {}", k, ".".repeat(max_key_len - k.len() + 3), v);
        }
    } else {
        for (k, _) in vers.iter() {
            println!("   {}", k);
        }
    }
}
