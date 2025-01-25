use crate::db::{ivec_to_string, Database};
use crate::scrape::Scraper;
use clap::ValueEnum;
use std::{cmp::Ordering, collections::HashMap};

#[derive(ValueEnum, Clone)]
pub enum SortBy {
    TimeNew,
    TimeOld,
    NameNew,
    NameOld,
}

pub(super) async fn handler(
    version: Option<String>,
    load_order: bool,
    display_time: bool,
    sort_by: Option<SortBy>,
) {
    let db = Database::new();
    if load_order {
        let scrap = Scraper::new().await;
        let all_opt_vers: Vec<&String> = scrap.get_all_opt_vers().iter().collect();
        let all_opt_vers_serial = bincode::serialize(&all_opt_vers).unwrap();
        db.get_cache_db()
            .insert("all_opt_ver", all_opt_vers_serial)
            .expect("Failed to insert Optifine version into database");
    }
    let order_from_db = db
        .get_cache_db()
        .get("all_opt_ver")
        .unwrap()
        .unwrap_or_default();
    let order: HashMap<String, usize> = bincode::deserialize::<Vec<String>>(&order_from_db)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let mut vers: Vec<(String, String)> = db
        .get_version_db()
        .iter()
        .filter_map(|e| match e {
            Ok((k, v)) => Some((ivec_to_string(&k), ivec_to_string(&v))),
            Err(_) => None,
        })
        .filter(|(k, _)| k.contains(version.as_deref().unwrap_or("")))
        .collect();
    match sort_by {
        None | Some(SortBy::NameNew) => {
            vers.sort_by_key(|(k, _)| order.get(k).copied().unwrap_or(usize::MAX))
        }
        Some(SortBy::NameOld) => vers.sort_by(|(ka, _), (kb, _)| {
            match (order.get(ka), order.get(kb)) {
                (Some(&va), Some(&vb)) => vb.cmp(&va), // reverse ordering
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        }),
        Some(SortBy::TimeNew) => vers.sort_by(|(_, va), (_, vb)| {
            let time_a = chrono::NaiveDateTime::parse_from_str(va, "%Y-%m-%d %H:%M:%S");
            let time_b = chrono::NaiveDateTime::parse_from_str(vb, "%Y-%m-%d %H:%M:%S");
            match (time_a, time_b) {
                (Ok(ta), Ok(tb)) => tb.cmp(&ta), // the larger, the former
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => Ordering::Equal,
            }
        }),
        Some(SortBy::TimeOld) => vers.sort_by(|(_, va), (_, vb)| {
            let time_a = chrono::NaiveDateTime::parse_from_str(va, "%Y-%m-%d %H:%M:%S");
            let time_b = chrono::NaiveDateTime::parse_from_str(vb, "%Y-%m-%d %H:%M:%S");
            match (time_a, time_b) {
                (Ok(ta), Ok(tb)) => ta.cmp(&tb), // the larger, the latter
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => Ordering::Equal,
            }
        }),
    }
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
