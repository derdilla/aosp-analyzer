use std::collections::HashMap;
use jwalk::DirEntry;
use crate::lang_stats::LangStats;
use crate::language::Language;

/// Code stats in a directory.
#[derive(Debug)]
pub struct CountContext {
    /// File extension and amount of lines in that extension.
    data: HashMap<Language, LangStats>
}

impl CountContext {
    pub fn new() -> Self {
        CountContext {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: DirEntry<((),())>) {
        let lang = Language::new(file.file_name.to_str().unwrap());
        if let Language::OTHER(_) = &lang {
            return; // TODO
        }

        let stats = self.data.entry(lang.clone())
            .or_insert(LangStats::new());
        stats.add(file.path().to_str().unwrap(), lang);
    }
}