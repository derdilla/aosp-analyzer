use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::DirEntry;
use crate::lang_stats::LangStats;
use crate::language::Language;

/// Code stats of a directory.
#[derive(Debug)]
pub struct CountContext {
    /// Contained files and directories.
    children: Vec<Box<dyn HasStats>>,
}

impl CountContext {
    pub fn new() -> Self {
        CountContext {
            children: vec![],
        }
    }

    pub fn insert_file(&mut self, file: DirEntry) {
        let file = SourceFile::new(file);
        self.children.push(Box::new(file));
    }

    pub fn insert_context(&mut self, dir: Self) {
        self.children.push(Box::new(dir));
    }
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    path: String,
    lang: Language,
    stats: LangStats,
}

impl SourceFile {
    pub fn new(file: DirEntry)  -> Self {
        let lang = Language::new(file.file_name().to_str().unwrap());
        let mut stats = LangStats::new();
        stats.add(file.path().to_str().unwrap(), &lang);
        SourceFile {
            path: file.path().as_path().to_str().unwrap().to_string(),
            lang,
            stats
        }
    }
}

pub trait HasStats: Debug {
    fn stats(&self) -> HashMap<Language, LangStats>;
}

impl HasStats for CountContext {
    fn stats(&self) -> HashMap<Language, LangStats> {
        let mut all_stats = HashMap::new();
        for file in &self.children {
            for (lang, file_stat) in file.stats() {
                let entry = all_stats.entry(lang).or_insert(LangStats::new());
                entry.join(&file_stat);
            }
        }

        all_stats
    }
}

impl HasStats for SourceFile {
    fn stats(&self) -> HashMap<Language, LangStats> {
        let mut map = HashMap::new();
        map.insert(self.lang.clone(), self.stats.clone());
        map
    }
}