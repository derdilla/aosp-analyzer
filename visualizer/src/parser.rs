use serde::Deserialize;
use std::{collections::HashMap, fs, ops::Add};

/// Data as extracted from statistics files generated by analyze.sh.
pub struct Data {
    pub core: HashMap<String, TokeiCodeStatistics>,
    pub devtools: HashMap<String, TokeiCodeStatistics>,
    pub sdks: HashMap<String, TokeiCodeStatistics>,
    pub thirdparty: HashMap<String, TokeiCodeStatistics>,
    pub userspace: HashMap<String, TokeiCodeStatistics>,
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

    fn load_file(p: String) -> HashMap<String, TokeiCodeStatistics> {
        println!("{p}");
        let content = fs::read_to_string(&p).unwrap();
        serde_json::from_str(content.as_str()).unwrap()
    }

    /// Create cummulative stats across all categories, with cleaned up languages
    pub fn lang_overview(&self) -> HashMap<String, TokeiCodeStatistics> {
        let total = self.compute_total();
        Self::filter_langs(total)
    }

    /// Accumulate data across categoies
    fn compute_total(&self) -> HashMap<String, TokeiCodeStatistics> {
        let mut map = HashMap::new();
        for x in vec![&self.core, &self.devtools, &self.sdks, &self.thirdparty, &self.userspace] {
            for (k, v) in x {
                let stats = map.entry(k.to_string())
                    .or_insert(TokeiCodeStatistics::default());
                stats.join(v);
            }
        }
        map
    }

    /// Filter languages to be "real" and actionable
    /// 
    /// ### Remove non-languages
    ///
    /// - Total
    /// 
    /// ### Join languages
    /// 
    /// - c++, c++ header, cpp
    fn filter_langs(mut data: HashMap<String, TokeiCodeStatistics>) -> HashMap<String, TokeiCodeStatistics> {
        // Remove excess data
        data.remove("Total");
        data.remove("XML");
        data.remove("HTML");
        data.remove("JSON");
        
        // Join langs
        let mut cpp_stats = TokeiCodeStatistics::default();
        for key in ["C++ Header", "C++", "Cpp"] {
            if let Some(stats) = data.get(key) {
                cpp_stats.join(stats);
                data.remove(key);
            }
        }
        data.insert(String::from("C++"), cpp_stats);

        let mut c_stats = TokeiCodeStatistics::default();
        for key in ["C Header", "C"] {
            if let Some(stats) = data.get(key) {
                c_stats.join(stats);
                data.remove(key);
            }
        }
        data.insert(String::from("C"), c_stats);

        data
        
    }
}
// FIXME: put files with test in file name seperate

#[derive(Debug, Deserialize)]
pub struct TokeiCodeStatistics {
    pub children: Option<HashMap<String, Vec<TokeiReport>>>,
    pub blanks: u32,
    pub code: u32,
    pub comments: u32,
    //pub inaccurate: Option<bool>,
    pub reports: Option<Vec<TokeiReport>>,
}

#[derive(Debug, Deserialize)]
pub struct TokeiReport {
    pub name: String,
    pub stats: TokeiStats,
}

#[derive(Debug, Deserialize)]
pub struct TokeiStats {
    pub blanks: u32,
    pub code: u32,
    pub comments: u32,
    pub blobs: HashMap<String, TokeiCodeStatistics>,
}

impl Default for TokeiCodeStatistics {
    fn default() -> Self {
        Self { children: Default::default(), blanks: Default::default(), code: Default::default(), comments: Default::default(), reports: Default::default() }
    }
}

impl TokeiCodeStatistics {
    fn join(&mut self, other: &TokeiCodeStatistics) {
            // TODO combined_children,children: self.children, 
            self.blanks += other.blanks;
            self.code += other.code;
            self.comments += other.comments;
            // TODO reports: self.reports,
    }
}
