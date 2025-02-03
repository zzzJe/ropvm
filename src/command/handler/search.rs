use crate::{db::Database, scrape::Scraper};

pub(super) async fn handler(version: Option<String>) {
    let scrape = Scraper::new().await;
    if let Some(version) = version {
        println!("📦 Avaliable Optifine version for {version}");
        let vers = scrape.get_opt_vers(&version[..]);
        let db = Database::new().get_version_db();
        for ver in vers {
            if db.get(ver).unwrap().is_some() {
                println!("🟢 {ver}");
            } else {
                println!("🔘 {ver}");
            }
        }
    } else {
        println!("📦 Avaliable Minecraft version");
        const COL_COUNT: usize = 6;
        let vers: Vec<&String> = scrape.get_mc_vers().iter().collect();
        // Use `vers.len() / COL_COUNT` will represent like this:
        // if vers.len() % COL_COUNT is 0
        //   -> 6 full columns
        // else
        //   -> 6 full columns + 1 remain column
        // But for smaller element, this method will cause some chaos
        // For example, when #element is 10, it will become 10 columns
        // By the fact that existed mc ver is large enough, choose this method for simplicity
        let vers_chucks: Vec<Vec<&String>> = vers
            .chunks(vers.len() / COL_COUNT)
            .map(|e| e.to_vec())
            .collect();
        let vers_repr = transpose(&vers_chucks);
        for row in vers_repr {
            for ver in row {
                print!("   {:<6}", ver);
            }
            println!();
        }
    }
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
    for row in matrix {
        for (j, val) in row.iter().enumerate() {
            transposed[j].push(val.clone());
        }
    }
    transposed
}
