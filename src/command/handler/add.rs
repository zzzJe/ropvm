use super::util::get_current_time;
use crate::{
    db::{ivec_to_string, Database, Tree},
    index::{destruct_input, parse, purify},
    scrape::Scraper,
};
use indexmap::IndexSet;
use std::path::Path;

pub(super) async fn handler(versions: Vec<String>) {
    if versions.is_empty() {
        // early return to avoid loading the pages
        return;
    }
    let scrap = Scraper::new().await;
    let db = Database::new();
    let ver_db = db.get_version_db();
    let conf_db = db.get_config_db();
    let cache_db = db.get_cache_db();

    let mut to_download = IndexSet::new();
    for ver in versions {
        match destruct_input(&ver) {
            // mc ver. exists
            Some((mc_ver, index)) if scrap.test_mc_ver(&mc_ver) => {
                let opt_vers = scrap.get_opt_vers(&mc_ver);
                let parsed_index = parse(&index, opt_vers.len());
                let purified_index = purify(parsed_index.0, opt_vers.len());
                to_download.extend(purified_index.iter().map(|&i| opt_vers[i - 1].clone()));
                for (raw, reason) in parsed_index.1 {
                    println!("❌ {reason} (at index '{raw}', '{mc_ver}[{raw}]')");
                }
            }
            // mc ver. not exists
            Some((mc_ver, _)) => println!("❌ No such Minecraft version '{mc_ver}'"),
            // opt ver. exists
            None if scrap.test_opt_ver(&ver) => {
                to_download.insert(ver.to_string());
            }
            // opt ver. not exists but it is a mc ver
            None if scrap.test_mc_ver(&ver) => {
                let first = scrap.get_opt_vers(&ver)[0].clone();
                to_download.insert(first);
            }
            // opt ver. not exists
            None => println!("❌ No such Optifine version '{ver}'"),
        }
    }

    async fn download(
        ver_db: Tree,
        conf_db: Tree,
        opt_ver: String,
        max_ver_len: usize,
    ) -> Result<(), ()> {
        let out_path = match conf_db.get("repo_dir").unwrap() {
            Some(ivec) => {
                let repo_dir = &ivec_to_string(&ivec);
                Path::new(repo_dir).join(format!("{opt_ver}.jar"))
            }
            None => Path::new("repo").join(format!("{opt_ver}.jar")),
        };
        let result = Scraper::download_opt_file(&opt_ver, &out_path).await;
        match result {
            Ok(_) => {
                let current_time = get_current_time();
                ver_db
                    .insert(&opt_ver, &current_time[..])
                    .unwrap_or_else(|_| {
                        panic!("Failed to insert Optifine version {opt_ver} into datebase")
                    });
                println!(
                    "✅ {opt_ver} {} success!",
                    ".".repeat(max_ver_len - opt_ver.len() + 3)
                );
                Ok(())
            }
            Err(_) => {
                println!(
                    "❌ {opt_ver} {} failed!",
                    ".".repeat(max_ver_len - opt_ver.len() + 3)
                );
                Err(())
            }
        }
    }

    let max_ver_len = to_download.iter().map(|e| e.len()).max();
    let to_download: Vec<_> = to_download
        .into_iter()
        .filter_map(|ver| {
            if ver_db.get(&ver[..]).unwrap().is_some() {
                println!(
                    "🔵 {ver} {} already exists!",
                    ".".repeat(max_ver_len.unwrap() - ver.len() + 3),
                );
                None
            } else {
                Some(ver)
            }
        })
        .collect();
    let futures: Vec<_> = to_download
        .into_iter()
        .map(|opt_ver| {
            tokio::spawn(download(
                ver_db.clone(),
                conf_db.clone(),
                opt_ver,
                max_ver_len.unwrap(),
            ))
        })
        .collect();
    let results = futures::future::join_all(futures).await;
    let recap = results.iter().fold((0, 0), |(succ, fail), item| {
        match item.as_ref().unwrap_or(&Err(())) {
            Ok(_) => (succ + 1, fail),
            Err(_) => (succ, fail + 1),
        }
    });
    if recap.0 > 1 || recap.1 > 0 {
        println!("👉 {} success / {} failed", recap.0, recap.1);
    }
    ver_db.flush_async().await.unwrap();

    // store opt version in db cache
    let all_opt_vers: Vec<&String> = scrap.get_all_opt_vers().iter().collect();
    let all_opt_vers_serial = bincode::serialize(&all_opt_vers).unwrap();
    cache_db
        .insert("all_opt_ver", all_opt_vers_serial)
        .expect("Failed to insert Optifine version into database");
}
