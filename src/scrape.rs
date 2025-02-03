use indexmap::{IndexMap, IndexSet};
use regex::Regex;
use scraper::{Html, Selector};
use std::{cell::RefCell, path::Path};
use tokio::io::AsyncWriteExt;

pub struct Scraper {
    dom: Html,
    mc_ver: RefCell<Option<IndexSet<String>>>,
    opt_all_ver: RefCell<Option<IndexSet<String>>>,
    opt_ver: RefCell<IndexMap<String, Vec<String>>>,
}

#[derive(Debug)]
pub enum ScrapeError {
    Reqwest,
    ScraperSelector,
    Io,
}

impl From<reqwest::Error> for ScrapeError {
    fn from(_error: reqwest::Error) -> ScrapeError {
        ScrapeError::Reqwest
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for ScrapeError {
    fn from(_error: scraper::error::SelectorErrorKind<'_>) -> ScrapeError {
        ScrapeError::ScraperSelector
    }
}

impl From<std::io::Error> for ScrapeError {
    fn from(_error: std::io::Error) -> ScrapeError {
        ScrapeError::Io
    }
}

impl Scraper {
    pub async fn new() -> Self {
        let with_url = Self::with_url("https://optifine.net/downloads").await;
        with_url.expect("Failed to connect to optifine.net")
    }
    async fn with_url(url: &'static str) -> Result<Self, ScrapeError> {
        let html_text = reqwest::get(url).await?.text().await?;
        Result::Ok(Self {
            dom: Html::parse_document(&html_text),
            mc_ver: RefCell::default(),
            opt_all_ver: RefCell::default(),
            opt_ver: RefCell::default(),
        })
    }
}

impl Scraper {
    pub fn get_mc_vers(&self) -> &IndexSet<String> {
        // The Optifine HTML structure is like this
        // body -> table -> tbody
        //   -> tr (header)
        //   -> tr (content) -> span.downloads
        //     -> h2 (mc vers)
        //     -> ... -> table.downloadTable -> tbody
        //       -> td (opt vers)
        //       -> td (download anchor)
        //       -> td (mirror)
        //       -> td (change log)
        //       -> td (forge)
        //       -> td (date)
        //   -> tr (footer)
        // The ".content span.downloads h2" is a path to mc vers manifest
        if self.mc_ver.borrow().is_none() {
            const SELECTOR_PATTERN: &str = ".content span.downloads h2";
            let selector = Selector::parse(SELECTOR_PATTERN).unwrap();
            // The mc vers are represented as "Minecraft a.b.c"
            // So here simply just remove "Minecraft " padding
            const VER_PAD_PATTERN: &str = "Minecraft ";
            let result = self
                .dom
                .select(&selector)
                .map(|e| e.text().collect::<Vec<_>>().join(" "))
                .map(|e| e.replace(VER_PAD_PATTERN, ""))
                .collect();
            self.mc_ver.borrow_mut().replace(result);
        }
        unsafe { (*self.mc_ver.as_ptr()).as_ref().unwrap() }
    }
    pub fn get_all_opt_vers(&self) -> &IndexSet<String> {
        if self.opt_all_ver.borrow().is_none() {
            const SELECTOR_PATTERN: &str = "table.downloadTable tr.downloadLine td.colMirror a";
            let selector = Selector::parse(SELECTOR_PATTERN).unwrap();
            // Note that we use `.unwrap()` all the way
            // If the file name structure changed, this will cause the cli to crash directly
            let opt_ver_re = Regex::new(r"OptiFine_(.*?).jar").unwrap();
            let result = self
                .dom
                .select(&selector)
                .filter_map(|e| {
                    let href = e.value().attr("href")?;
                    let captures = opt_ver_re.captures(href)?;
                    let version = captures.get(1)?.as_str().to_string();
                    Some(version)
                })
                .collect();
            self.opt_all_ver.borrow_mut().replace(result);
        }
        unsafe { (*self.opt_all_ver.as_ptr()).as_ref().unwrap() }
    }
    pub fn get_opt_vers(&self, mc_ver: &str) -> &[String] {
        if self.opt_ver.borrow().get(mc_ver).is_none() {
            let mc_ver_head_pat = format!("{mc_ver}_");
            let result = self
                .get_all_opt_vers()
                .iter()
                .filter(|s| s[..].starts_with(&mc_ver_head_pat))
                .map(|s| s.to_string())
                .collect();
            self.opt_ver.borrow_mut().insert(mc_ver.to_string(), result);
        }
        unsafe { (*self.opt_ver.as_ptr()).get(mc_ver).unwrap() }
    }
    pub fn test_mc_ver(&self, mc_ver: &str) -> bool {
        self.get_mc_vers().get(mc_ver).is_some()
    }
    pub fn test_opt_ver(&self, opt_ver: &str) -> bool {
        self.get_all_opt_vers().get(opt_ver).is_some()
    }
}

impl Scraper {
    async fn get_download_stream(opt_ver: &str) -> Result<String, ScrapeError> {
        let url = format!(
            "https://optifine.net/adloadx?f={file_header}OptiFine_{opt_ver}.jar",
            file_header = if opt_ver.contains("pre") {
                "preview_"
            } else {
                ""
            }
        );
        let html_text = reqwest::get(url).await?.text().await?;
        let dom = Html::parse_document(&html_text);
        const SELECTOR_PATTERN: &str = "table.tableDownload span#Download a";
        let selector = Selector::parse(SELECTOR_PATTERN).unwrap();
        let stream = dom
            .select(&selector)
            .map(|e| e.value().attr("href").unwrap())
            .next()
            .unwrap();
        Ok(format!("https://optifine.net/{stream}"))
    }
    pub async fn download_opt_file(opt_ver: &str, out_path: &Path) -> Result<(), ScrapeError> {
        let stream_url = Self::get_download_stream(opt_ver).await?;
        let client = reqwest::Client::new();
        let mut response = client.get(stream_url).send().await?;
        // automaticall create nessasary parent directory
        let parent_dir = out_path.parent().unwrap();
        tokio::fs::create_dir_all(parent_dir).await?;
        // the output dir
        let mut file = tokio::fs::File::create(out_path).await?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }
        Ok(())
    }
}
