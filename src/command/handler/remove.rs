use crate::db::{ivec_to_string, Database, Tree};
use std::{collections::HashSet, io::Write, path::Path, sync::Arc};
use tokio::fs;

pub(super) async fn handler(patterns: Vec<String>) {
    if patterns.is_empty() {
        return;
    }
    let db = Database::new();
    let ver_db = db.get_version_db();
    let conf_db = db.get_config_db();
    let current_files: Vec<String> = ver_db
        .iter()
        .map(|e| e.expect("Failed to get version from db"))
        .map(|(k, _)| ivec_to_string(&k))
        .collect();
    let to_delete = gather_2delete_files(current_files, &patterns);
    if to_delete.is_empty() {
        return;
    } else if to_delete.len() > 1 {
        print!(
            "🔰 This operation will delete {} files, keep going? [y/N] ",
            to_delete.len()
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input != "Y" && input != "y" {
            return;
        }
    }
    let max_display_len = to_delete.iter().map(|s| s.len()).max().unwrap();
    let base_path = match conf_db.get("repo_dir").unwrap() {
        Some(ivec) => ivec_to_string(&ivec),
        None => "repo".to_string(),
    };
    let base_path = Arc::new(base_path);
    let futures = to_delete
        .into_iter()
        .map(|name| (ver_db.clone(), base_path.clone(), Arc::new(name)))
        .map(|(ver_db, base_path, name)| {
            tokio::spawn(async move {
                handle_one_file(ver_db, base_path, name.clone())
                    .await
                    .map(|_| {
                        println!(
                            "✅ {} {} success!",
                            name,
                            ".".repeat(max_display_len - name.len() + 3)
                        )
                    })
                    .map_err(|_| {
                        println!(
                            "❌ {} {} failed!",
                            name,
                            ".".repeat(max_display_len - name.len() + 3)
                        )
                    })
            })
        });
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
}

fn gather_2delete_files(current_files: Vec<String>, patterns: &Vec<String>) -> HashSet<String> {
    let mut to_delete = HashSet::new();
    'outer: for filename in current_files {
        for pat in patterns {
            if filename.contains(pat) {
                to_delete.insert(filename);
                continue 'outer;
            }
        }
    }
    to_delete
}

async fn handle_one_file(db: Tree, base: Arc<String>, name: Arc<String>) -> Result<(), String> {
    let filename = format!("{}.jar", &*name);
    let file = Path::new(base.as_ref()).join(&filename);
    fs::remove_file(file)
        .await
        .map_err(|_| "Failed to remove file".to_string())
        .and_then(|_| {
            db.remove(&*name)
                .map_err(|_| "Failed to remove entry in db".to_string())
        })?;
    Ok(())
}
