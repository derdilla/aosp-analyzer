use std::collections::HashMap;
use jwalk::DirEntry;
use crate::lang_stats::LangStats;
use crate::language::Language;

/// Code stats in a directory.
#[derive(Debug)]
pub struct CountContext {
    /// Path of the folder or file that is counted.
    path: String,
    /// File extension and amount of lines in that extension.
    data: HashMap<Language, LangStats>
}

impl CountContext {
    pub fn new() -> Self {
        CountContext {
            path: String::new(),
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: DirEntry<((),())>) {
        let lang = Language::new(file.file_name.to_str().unwrap());
        if let Language::OTHER(_) = &lang {
            return; // TODO
        }

        let stats = self.data.entry(lang)
            .or_insert(LangStats::new(Language::new(file.file_name.to_str().unwrap())));
        if !stats.add(file.path().to_str().unwrap()) {
            // TODO: error propagation
        }
    }
}