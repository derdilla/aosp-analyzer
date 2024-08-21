use serde::Deserialize;
use std::{collections::HashMap, fs};

pub struct Data {
    pub core: HashMap<String, CodeStatistics>,
    pub devtools: HashMap<String, CodeStatistics>,
    pub sdks: HashMap<String, CodeStatistics>,
    pub thirdparty: HashMap<String, CodeStatistics>,
    pub userspace: HashMap<String, CodeStatistics>,
    // TODO: userspace
}

impl Data {
    pub fn load(stats_path: &str) -> Self {
        Data {
            core: Self::load_file(format!("{stats_path}core.json")),
            devtools: Self::load_file(format!("{stats_path}devtools.json")),
            sdks: Self::load_file(format!("{stats_path}sdks.json")),
            thirdparty: Self::load_file(format!("{stats_path}thirdparty.json")),
            userspace: Self::load_file(format!("{stats_path}userspace.json")),
        }
    }

    fn load_file(p: String) -> HashMap<String, CodeStatistics> {
        println!("{p}");
        let content = fs::read_to_string(&p).unwrap();
        serde_json::from_str(content.as_str()).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct CodeStatistics {
    //pub children: Option<HashMap<String, Vec<Report>>>,
    pub blanks: u32,
    pub code: u32,
    pub comments: u32,
    //pub inaccurate: Option<bool>,
    //pub reports: Option<Vec<Report>>,
}
/*
pub struct Report {
    pub name: String,
    pub stats: Stats,
pub struct Stats {
    pub blobs: HashMap<String, CodeStatistics>,
*/