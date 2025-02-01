use crate::db::{self, ivec_to_string, Database, Tree};
use std::{
    io::Write,
    path::Path,
    process::{ExitStatus, Stdio},
};
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::Command,
};

const TEST_JAR: &'static [u8] = include_bytes!("../../../test_resources/java/Test.jar");

pub(super) async fn handler(
    minecraft_dir: Option<String>,
    java_path: Option<String>,
    repo_dir: Option<String>,
    test: bool,
) {
    let scaffolding = minecraft_dir.is_none() && java_path.is_none() && repo_dir.is_none();
    let db = Database::new().get_config_db();

    let mut tasks = vec![];

    if let Some(mc_dir) = minecraft_dir {
        let db = db.clone();
        tasks.push(tokio::spawn(async move {
            if config_mc_dir(db.clone(), &mc_dir).is_err() {
                println!("❌ Failed to config minecraft-dir");
            } else if test {
                match test_mc_dir(db.clone()).await {
                    Ok(output) => println!("{output}"),
                    Err(reason) => println!("{reason}"),
                }
            }
        }));
    }
    if let Some(java_path) = java_path {
        let db = db.clone();
        tasks.push(tokio::spawn(async move {
            if config_java(db.clone(), &java_path).is_err() {
                println!("❌ Failed to config java-path");
            } else if test {
                match test_java(db.clone()).await {
                    Ok(output) => println!("{output}"),
                    Err(reason) => println!("{reason}"),
                }
            }
        }))
    }
    if let Some(repo) = repo_dir {
        let db = db.clone();
        tasks.push(tokio::spawn(async move {
            if config_repo(db.clone(), &repo).is_err() {
                println!("❌ Failed to config repo-dir")
            } else if test {
                match test_repo(db.clone()).await {
                    Ok(output) => println!("{output}"),
                    Err(reason) => println!("{reason}"),
                }
            }
        }));
    }
    if scaffolding {
        if test {
            let db = db.clone();

            async fn judge<F, Fut>(db: Tree, f: F)
            where
                F: Fn(Tree) -> Fut,
                Fut: std::future::Future<Output = Result<String, String>>,
            {
                match f(db.clone()).await {
                    Ok(output) => println!("{output}"),
                    Err(reason) => println!("{reason}"),
                }
            }

            tasks.push(tokio::spawn(judge(db.clone(), test_mc_dir)));
            tasks.push(tokio::spawn(judge(db.clone(), test_java)));
            tasks.push(tokio::spawn(judge(db.clone(), test_repo)));
        } else {
            let db = db.clone();

            tasks.push(tokio::spawn(async move {
                println!("⭐ opvm config scaffolding ⭐");
                let input = read_line("👉 --minecraft-dir: ");
                if config_mc_dir(db.clone(), &input).is_ok() {
                    println!("✅ set to '{input}'");
                } else {
                    println!("❌ Failed to config minecraft-dir")
                }
                let input = read_line("👉 --java-path: ");
                if config_java(db.clone(), &input).is_ok() {
                    println!("✅ set to '{input}'");
                } else {
                    println!("❌ Failed to config java-path")
                }
                let input = read_line("👉 --repo-dir: ");
                if config_repo(db.clone(), &input).is_ok() {
                    println!("✅ set to '{input}'");
                } else {
                    println!("❌ Failed to config repo-dir")
                }
            }));
        }
    }
    let _results = futures::future::join_all(tasks).await;
    db.flush_async().await.expect("Database flush failed");
}

fn read_line(print: &str) -> String {
    let mut input = String::new();
    print!("{print}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn config_mc_dir(db: Tree, mc_dir: &String) -> db::Result<()> {
    if mc_dir.is_empty() {
        db.remove("mc_dir")?;
    } else {
        db.insert("mc_dir", mc_dir.as_bytes())?;
    }
    Ok(())
}

async fn test_mc_dir(db: Tree) -> Result<String, String> {
    let entry = db.get("mc_dir").unwrap();
    // 1. test if entry exists
    if entry.is_none() {
        return Err(format!("🛑 minecraft-dir has not been configured yet"));
    }
    // 2. test if entry is a valid dir
    let path_string = ivec_to_string(entry.as_ref().unwrap());
    let path = Path::new(&path_string);
    if !is_readable_dir(&path).await {
        return Err(format!(
            "🛑 The path '{path_string}' is not a readable directory"
        ));
    }
    // 3. test if entry is a mc dir
    let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
    let mut read_dir = fs::read_dir(path).await.unwrap();
    let mut subdir = vec![];
    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|_| "IO error occurs when reading mc dir")?
    {
        if entry.path().is_dir() {
            subdir.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    let subdir = subdir; // shadowing to change mutability

    let match_trait = ["resourcepacks", "saves", "versions"]
        .iter()
        .all(|d| subdir.contains(&d.to_string()));
    if dir_name != ".minecraft" || !match_trait {
        return Err(format!(
            "🛑 The path '{path_string}' seems not to be a minecraft directory"
        ));
    }
    Ok(format!("✅ minecraft-dir: '{path_string}'"))
}

fn config_java(db: Tree, java_path: &String) -> db::Result<()> {
    if java_path.is_empty() {
        db.remove("java_path")?;
    } else {
        db.insert("java_path", java_path.as_bytes())?;
    }
    Ok(())
}

async fn test_java(db: Tree) -> Result<String, String> {
    let entry = db.get("java_path").unwrap();

    create_test_jar_and_dir_if_not_exist()
        .await
        .map_err(|_| "Failed to create .test_resources/Test.jar".to_string())?;

    if let Some(java) = entry {
        let java = ivec_to_string(&java);
        let status = open_process_to_test_java(&java).await;
        match status {
            Ok(Ok(exit)) if exit.success() => Ok(format!("✅ java-path: '{java}'")),
            _ => Err(format!("🛑 Given java-path '{java}' is invalid")),
        }
    } else {
        let status = open_process_to_test_java("java").await;
        match status {
            Ok(Ok(exit)) if exit.success() => Ok(format!("✅ java-path: default (javaw)")),
            _ => Err(format!("🛑 No avaliable javaw machine be found")),
        }
    }
}

async fn create_test_jar_and_dir_if_not_exist() -> Result<(), std::io::Error> {
    let filepath = Path::new(".test_resources/Test.jar");
    fs::create_dir_all(".test_resources").await?;
    if !filepath.exists() {
        fs::write(filepath, TEST_JAR).await?;
    }
    Ok(())
}

async fn open_process_to_test_java(java: &str) -> Result<Result<ExitStatus, ()>, std::io::Error> {
    let mut child = Command::new(java)
        .arg("-jar")
        .arg(".test_resources/Test.jar")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut writer = BufWriter::new(child.stdin.take().expect("❌ Failed to open stdin"));
    writer.write_all(b"1\n").await?;
    writer.write_all(b"2\n").await?;
    writer.flush().await?;

    let mut stdout = BufReader::new(child.stdout.take().expect("❌ Failed to open stdout"));
    let mut output = String::new();
    stdout.read_line(&mut output).await?;

    if output != "Hello World! 1 + 2 = 3!" {
        return Ok(Err(()));
    }

    let status = child.wait().await?;

    Ok(Ok(status))
}

fn config_repo(db: Tree, repo_dir: &String) -> db::Result<()> {
    if repo_dir.is_empty() {
        db.remove("repo_dir")?;
    } else {
        db.insert("repo_dir", repo_dir.as_bytes())?;
    }
    Ok(())
}

async fn test_repo(db: Tree) -> Result<String, String> {
    let entry = db.get("repo_dir").unwrap();

    if let Some(repo) = entry {
        let repo_string = ivec_to_string(&repo);
        let repo = Path::new(&repo_string);

        if is_readable_dir(repo).await && is_writable_dir(repo).await {
            Ok(format!("✅ repo-dir: '{repo_string}'"))
        } else {
            Err(format!("🛑 Given repo-dir '{repo_string}' is invalid"))
        }
    } else {
        // automaticall create nessasary parent directory
        let repo_dir = Path::new("repo");
        tokio::fs::create_dir_all(repo_dir)
            .await
            .map_err(|_| "❌ Failed to create default repo directory")?;
        if is_writable_dir(repo_dir).await {
            Ok("✅ repo-dir: default (repo/)".to_string())
        } else {
            Err("🛑 Cannot add/remove file at default local repo".to_string())
        }
    }
}

async fn is_readable_dir(path: &Path) -> bool {
    path.exists() && path.is_dir() && fs::read_dir(path).await.is_ok()
}

async fn is_writable_dir(path: &Path) -> bool {
    if !path.exists() || !path.is_dir() {
        return false;
    }
    match fs::File::create(path.join(".writable_check")).await {
        Ok(_) => {
            let _ = fs::remove_file(path.join(".writable_check")).await;
            true
        }
        Err(_) => false,
    }
}
