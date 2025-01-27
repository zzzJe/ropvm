use super::{util::sort_vers, ListSortBy};
use crate::{
    db::{ivec_to_string, Database, Tree},
    index::{destruct_input, parse, ParsedRange},
};
use regex::Regex;
use std::{path::Path, process::Stdio};
use tokio::{fs, io::AsyncReadExt, process::Command};

enum InvokeError {
    ExitFail,
    InvokeFail,
    UserCancel,
    McDirNotFound,
    LauncherProfileRead,
}

pub(super) async fn handler(version: String) {
    let db = Database::new();
    let ver_db = db.get_version_db();
    let conf_db = db.get_config_db();
    let cache_db = db.get_cache_db();

    let to_apply = match destruct_input(&version) {
        Some((mc_ver, index)) => {
            let pat = format!("{mc_ver}_");
            let vers_unsorted: Vec<(String, String)> = ver_db
                .iter()
                .filter_map(|e| {
                    let k = ivec_to_string(&e.unwrap().0);
                    k.contains(&pat).then_some((k, String::new()))
                })
                .collect();
            let mut vers = sort_vers(vers_unsorted, &cache_db, ListSortBy::NameNew);
            let (parsed_valid_index, parsed_invalid_index) = parse(&index, vers.len());
            match (parsed_valid_index.get(0), parsed_invalid_index) {
                (Some(ParsedRange::Single(i)), invalid)
                    if vers.len() > 0 && invalid.is_empty() && parsed_valid_index.len() == 1 =>
                {
                    Some(vers.swap_remove(*i - 1).0)
                }
                _ => None,
            }
        }
        // use the one begin first matched
        None => {
            let mut vers: Vec<(String, String)> = ver_db
                .iter()
                .filter_map(|e| {
                    let k = ivec_to_string(&e.unwrap().0);
                    k.contains(&version).then(|| (k, String::new()))
                })
                .collect();
            (!vers.is_empty()).then(|| vers.swap_remove(0).0)
        }
    };
    match to_apply {
        Some(ver) => {
            let result = invoke_gui_and_check(&conf_db, &ver).await;
            if result.is_ok() {
                println!("✅ {ver} success!");
                cache_db
                    .insert("applied_ver", ver.as_bytes())
                    .expect("Failed to write applied version into db");
            } else {
                println!("🛑 {ver} failed to apply!");
            }
        }
        None => {
            println!("❌ No version matches '{version}' in local repo");
        }
    }
}

async fn invoke_gui_and_check(conf_db: &Tree, ver: &str) -> Result<(), InvokeError> {
    let ver = format!("{ver}.jar");
    let mc_dir = conf_db
        .get("mc_dir")
        .unwrap()
        .map(|ivec| ivec_to_string(&ivec))
        .ok_or(InvokeError::McDirNotFound)?;
    let launcher_profile_path = Path::new(&mc_dir).join("launcher_profiles.json");
    let java = conf_db
        .get("java_path")
        .unwrap()
        .map(|ivec| ivec_to_string(&ivec))
        .unwrap_or("javaw".to_string());
    let repo = conf_db
        .get("repo_dir")
        .unwrap()
        .map(|ivec| ivec_to_string(&ivec))
        .unwrap_or("repo".to_string());
    let path = Path::new(&repo).join(ver);
    let mut child = Command::new(java)
        .arg("-jar")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|_| InvokeError::InvokeFail)?;

    let profile_before = get_launcher_profile(&launcher_profile_path).await?;
    let status = child.wait().await.map_err(|_| InvokeError::InvokeFail)?;
    let profile_after = get_launcher_profile(&launcher_profile_path).await?;

    let modify_date_before = get_modify_time(&profile_before);
    let modify_date_after = get_modify_time(&profile_after);

    match (modify_date_before, modify_date_after) {
        (Some(d1), Some(d2)) if d1 == d2 => Err(InvokeError::UserCancel),
        (Some(_), None) => Err(InvokeError::UserCancel),
        _ => Ok(()),
    }?;

    status.success().then(|| ()).ok_or(InvokeError::ExitFail)
}

async fn get_launcher_profile(launcher_profile_path: &Path) -> Result<String, InvokeError> {
    let mut launcher_profile = fs::File::open(launcher_profile_path)
        .await
        .map_err(|_| InvokeError::LauncherProfileRead)?;
    let mut buf = String::new();
    launcher_profile
        .read_to_string(&mut buf)
        .await
        .map_err(|_| InvokeError::LauncherProfileRead)?;
    Ok(buf)
}

fn get_modify_time(raw_profile: &str) -> Option<&str> {
    let pat = Regex::new(r#""OptiFine"\s*:\s*\{[^}]*"lastUsed"\s*:\s*"([^"]+)""#).unwrap();
    if let Some(c) = pat.captures(raw_profile).and_then(|c| c.get(1)) {
        Some(c.as_str())
    } else {
        None
    }
}
