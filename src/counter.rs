use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use crate::file_stats::FileStats;
use crate::lang_stats::LangStats;
use crate::language::Language;

/// Code stats of a directory.
#[derive(Debug)]
pub struct CountContext {
    pub dir_name: String,
    /// Contained files and directories.
    pub children: Vec<Box<dyn HasStats>>,
}

impl CountContext {
    pub fn new(dir_name: String) -> Self {
        CountContext {
            dir_name,
            children: vec![],
        }
    }

    /// Add stats of a non-ignored file
    pub fn insert_file(&mut self, file: &PathBuf) {
        let file = SourceFile::new(file);
        if let Some(file) = file {
            self.children.push(Box::new(file));
        }

    }

    pub fn insert_context(&mut self, dir: Self) {
        if dir.children.len() == 1 {
            if let Some(mut file) = dir.children.first().unwrap().file() {
                if dir.dir_name != file.file_name {
                    file.file_name = dir.dir_name + "/" + file.file_name.as_str();
                }
                self.children.push(Box::new(file));
            } else {
                self.children.push(Box::new(dir));
            }
        } else {
            self.children.push(Box::new(dir));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Clone, Debug)]
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

    fn file(&self) -> Option<SourceFile>;

    fn context<'a>(&'a self) -> Option<&'a CountContext>;
    fn name(&self) -> String;
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

    fn file(&self) -> Option<SourceFile> {
        None
    }

    fn context<'a>(&'a self) -> Option<&'a CountContext> {
        Some(self)
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

    fn file(&self) -> Option<SourceFile> {
        Some(self.clone())
    }

    fn context<'a>(&'a self) -> Option<&'a CountContext> {
        None
    }

    fn name(&self) -> String {
        self.file_name.to_string()
    }
}