use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use serde::{Serialize};
use crate::file_stats::FileStats;
use crate::lang_stats::LangStats;
use crate::language::Language;

/// Code stats of a directory.
#[derive(Debug, Clone, Serialize)]
pub struct CountContext {
    pub dir_name: String,
    pub files: Vec<SourceFile>,
    pub dirs: Vec<CountContext>,
}

impl CountContext {
    pub fn new(dir_name: String) -> Self {
        CountContext {
            dir_name,
            files: vec![],
            dirs: vec![],
        }
    }

    pub fn insert_context(&mut self, dir: Self) {
        // TODO: is optimization to remove dirs with only one entry from tree beneficial ("foo"/"bar" -> "foo/bar") ?
        self.dirs.push(dir);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceFile {
    file_name: String,
    pub lang: Language,
    pub code_stats: Option<FileStats>,
    pub test_stats: Option<FileStats>,
}

impl SourceFile {
    pub fn new(file: &PathBuf) -> Option<Self> {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_path = file.to_str().unwrap();
        let lang = Language::new(file_name)?;
        let stats = FileStats::new(file_path, &lang);
        let is_test = file_path.contains("test");
        Some(SourceFile {
            file_name: file_name.to_string(),
            lang,
            code_stats: if is_test { None } else { Some(stats) },
            test_stats: if !is_test { None } else { Some(stats) },
        })
    }
}

pub trait HasStats: Debug + Send {
    fn stats(&self) -> HashMap<Language, LangStats>;

    fn name(&self) -> String;
}

impl HasStats for CountContext {
    fn stats(&self) -> HashMap<Language, LangStats> {
        let mut all_stats = HashMap::new();
        for file in &self.files {
            for (lang, file_stat) in file.stats() {
                let entry = all_stats.entry(lang).or_insert(LangStats::new());
                entry.join(&file_stat);
            }
        }

        all_stats
    }

    fn name(&self) -> String {
        self.dir_name.to_string()
    }
}

impl HasStats for SourceFile {
    fn stats(&self) -> HashMap<Language, LangStats> {
        let mut map = HashMap::new();
        let mut stats = LangStats::new();
        stats.add(&self);
        map.insert(self.lang.clone(), stats);
        map
    }

    fn name(&self) -> String {
        self.file_name.to_string()
    }
}