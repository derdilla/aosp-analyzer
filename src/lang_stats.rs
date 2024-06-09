use std::{fs};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use crate::counter::SourceFile;
use crate::file_stats::FileStats;
use crate::language::Language;
use crate::language::Language::OTHER;

/// Detailed statistics on source code.
#[derive(Clone, Debug, Serialize)]
pub struct LangStats {
    // Total counts are maintained to be able to verify data consistency.
    total_files: u16,
    code_files: u16,
    test_files: u16,
    total_lines: u32,
    /// Number of non-test lines of code. (includes comments or empty)
    code_lines: u32,
    /// Number of non-empty comment lines of code outside tests.
    code_comment_lines: u32,
    /// Number of empty lines of code outside tests.
    code_empty_lines: u32,
    /// Lines of code in tests (includes comments and empty).
    test_lines: u32,
    /// Number of non-empty comment lines of code in tests.
    test_comment_lines: u32,
    /// Number of empty lines of code in tests.
    test_empty_lines: u32,
}

impl LangStats {
    pub fn new() -> Self {
        LangStats {
            total_files: 0,
            code_files: 0,
            test_files: 0,
            total_lines: 0,
            code_lines: 0,
            code_comment_lines: 0,
            code_empty_lines: 0,
            test_lines: 0,
            test_comment_lines: 0,
            test_empty_lines: 0,
        }
    }

    pub fn add(&mut self, file: &SourceFile) {
        self.total_files += 1;
        if file.test_stats.is_some() {
            self.test_files += 1;
        }
        if file.code_stats.is_some() {
            self.code_files += 1;
        }
        // TODO: implement
    }

    pub fn join(&mut self, other: &Self) {
        self.total_files += other.total_files;
        self.code_files += other.code_files;
        self.test_files += other.test_files;
        self.total_lines += other.total_lines;
        self.code_lines += other.code_lines;
        self.code_comment_lines += other.code_comment_lines;
        self.code_empty_lines += other.code_empty_lines;
        self.test_lines += other.test_lines;
        self.test_comment_lines += other.test_comment_lines;
        self.code_empty_lines += other.code_empty_lines;
    }


}
