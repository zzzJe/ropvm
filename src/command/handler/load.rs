use super::util::get_current_time;
use crate::db::{ivec_to_string, Database, Result, Tree};
use std::collections::HashSet;
use tokio::fs;

pub(super) async fn handler() {
    let db = Database::new();
    let conf_db = db.get_config_db();
    let repo_path = match conf_db.get("repo_dir").unwrap() {
        Some(ivec) => ivec_to_string(&ivec),
        None => "repo".to_string(),
    };
    let existed_jars = get_exised_jars(&repo_path).await;
    let ver_db = db.get_version_db();
    match sync_vec_to_db(ver_db, existed_jars) {
        Ok(_) => println!("✅ Load success!"),
        Err(_) => println!("❌ Load failed!"),
    }
}

async fn get_exised_jars(repo_path: &String) -> Vec<String> {
    let mut readdir = fs::read_dir(repo_path)
        .await
        .expect("Failed to read repo dir");
    let mut existed_jars = vec![];
    while let Some(entry) = readdir.next_entry().await.expect("Failed to read entry") {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("jar") {
            continue;
        }
        let filename = path.file_stem();
        if let Some(filename) = filename {
            existed_jars.push(filename.to_string_lossy().to_string());
        }
    }
    existed_jars
}

fn sync_vec_to_db(db: Tree, vec: Vec<String>) -> Result<()> {
    let mut keys_in_db = HashSet::new();
    for entry in db.iter() {
        let (k, _) = entry?;
        keys_in_db.insert(ivec_to_string(&k));
    }
    let vec_set: HashSet<String> = vec.into_iter().collect();

    let to_remove: Vec<&String> = keys_in_db.difference(&vec_set).collect();
    let to_insert: Vec<&String> = vec_set.difference(&keys_in_db).collect();

    for key in to_remove {
        db.remove(key)?;
    }
    for key in to_insert {
        let time = get_current_time();
        db.insert(key, &time[..])?;
    }
    Ok(())
}
