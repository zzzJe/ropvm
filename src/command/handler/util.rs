use super::list::SortBy;
use crate::db::Tree;
use std::{cmp::Ordering, collections::HashMap};

pub(super) fn get_current_time() -> String {
    let date = chrono::Local::now();
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(super) fn sort_vers(
    vers: Vec<(String, String)>,
    cache_db: &Tree,
    sort_by: SortBy,
) -> Vec<(String, String)> {
    let order_from_db = cache_db.get("all_opt_ver").unwrap().unwrap_or_default();
    let order: HashMap<String, usize> = bincode::deserialize::<Vec<String>>(&order_from_db)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let mut vers = vers;
    match sort_by {
        SortBy::NameNew => vers.sort_by_key(|(k, _)| order.get(k).copied().unwrap_or(usize::MAX)),
        SortBy::NameOld => vers.sort_by(|(ka, _), (kb, _)| {
            match (order.get(ka), order.get(kb)) {
                (Some(&va), Some(&vb)) => vb.cmp(&va), // reverse ordering
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        }),
        SortBy::TimeNew => vers.sort_by(|(_, va), (_, vb)| {
            let time_a = chrono::NaiveDateTime::parse_from_str(va, "%Y-%m-%d %H:%M:%S");
            let time_b = chrono::NaiveDateTime::parse_from_str(vb, "%Y-%m-%d %H:%M:%S");
            match (time_a, time_b) {
                (Ok(ta), Ok(tb)) => tb.cmp(&ta), // the larger, the former
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => Ordering::Equal,
            }
        }),
        SortBy::TimeOld => vers.sort_by(|(_, va), (_, vb)| {
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
    vers
}
