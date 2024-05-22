use std::{fs};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::language::Language;
use crate::language::Language::OTHER;

/// Detailed statistics on source code.
#[derive(Debug)]
pub struct LangStats {
    // Total counts are maintained to be able to verify data consistency.
    total_files: u64,
    code_files: u64,
    test_files: u64,
    total_lines: u128,
    /// Number of non-test lines of code. (includes comments or empty)
    code_lines: u128,
    /// Number of non-empty comment lines of code outside tests.
    code_comment_lines: u128,
    /// Number of empty lines of code outside tests.
    code_empty_lines: u128,
    /// Lines of code in tests (includes comments and empty).
    test_lines: u128,
    /// Number of non-empty comment lines of code in tests.
    test_comment_lines: u128,
    /// Number of empty lines of code in tests.
    test_empty_lines: u128,
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

    /// Counts stats in a file and adds them.
    ///
    /// When the file format doesn't match or the  no stats are added and false is returned.
    pub fn add(&mut self, path: &str, lang: Language) {
        self.total_files += 1;
        match lang {
            Language::JAVA => self.analyze_java(path),
            Language::KOTLIN => self.analyze_java(path),
            Language::C => self.analyze_java(path),
            Language::CPP => self.analyze_java(path),
            Language::RUST => self.analyze_java(path), // FIXME: ignoring in file tests
            Language::PYTHON => self.analyze_bash(path), // FIXME: ignoring multiline string comments and tests
            Language::GRADLE => self.analyze_java(path),
            Language::CMAKE => self.analyze_bash(path), // https://cmake.org/cmake/help/v3.1/manual/cmake-language.7.html#comments
            Language::MAKEFILE => self.analyze_bash(path),
            Language::ASSEMBLY => self.analyze_general(path, CommentStyle::UNKNOWN), // TODO: implement
            OTHER(_) => self.analyze_general(path, CommentStyle::UNKNOWN),
        }
    }

    /// Analyze a undifferentiated file.
    fn analyze_general(&mut self, path: &str, comments: CommentStyle) {
        let lines = count_differentiated_lines(&path, comments);
        if path.contains("test") {
            self.test_files += 1;
            if let Some(lines) = lines {
                self.total_lines += lines.0 as u128;
                self.test_lines += lines.0 as u128;
                self.test_comment_lines += lines.1 as u128;
                self.test_empty_lines += lines.2 as u128;
            }
            // Other line types can't be differentiated.
        } else {
            self.code_files += 1;
            if let Some(lines) = lines {
                self.total_lines += lines.0 as u128;
                self.code_lines += lines.0 as u128;
                self.code_comment_lines += lines.1 as u128;
                self.code_empty_lines += lines.2 as u128;
            }
        }
    }

    fn analyze_java(&mut self, path: &str) {
        self.analyze_general(path, CommentStyle::C)
    }

    fn analyze_bash(&mut self, path: &str) {
        let lines = count_differentiated_lines(path, CommentStyle::BASH);
        self.code_files += 1;
        if let Some(lines) = lines {
            self.total_lines += lines.0 as u128;
            self.code_lines += lines.0 as u128;
            self.code_comment_lines += lines.1 as u128;
            self.code_empty_lines += lines.2 as u128;
        }
    }
}

enum CommentStyle {
    // Comments between "#" and "\n".
    BASH,
    // Comments between "//" and "\n or "/*" and "*/".
    C,
    // No comment counting.
    UNKNOWN,
}

/// Counts all_lines, comment_lines and empty_lines in a [file].
///
/// Counts are returned in the aforementioned order.
///
/// Comment borders examples:
/// "//" -> "\n"
/// "/*" -> "*/"
fn count_differentiated_lines(file: &str, comments: CommentStyle) -> Option<(usize, usize, usize)> {
    static EMPTY_LINES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n[\s\t\r]*\n").unwrap());
    static BASH_COMMENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n\s*#.*\n").unwrap());
    static C_COMMENT_SINGLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n\s*//.*\n").unwrap());
    static C_COMMENT_MULTI_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/\*(?:.|\n)*?\*/").unwrap());

    let content = fs::read_to_string(file).ok()?;

    let total_lines = content.matches("\n").count();
    let empty_lines = EMPTY_LINES_RE.find_iter(&content).count();
    let comment_lines: usize = match comments {
        CommentStyle::BASH => {
            BASH_COMMENT_RE.find_iter(&content).count()
        },
        CommentStyle::C => {
            let multi_comments: usize = C_COMMENT_MULTI_RE
                .find_iter(&content)
                .map(|m| m.as_str().matches('\n').count())
                .sum();
            C_COMMENT_SINGLE_RE.find_iter(&content).count() + multi_comments
        },
        CommentStyle::UNKNOWN => 0,
    };

    Some((total_lines, comment_lines, empty_lines))
}