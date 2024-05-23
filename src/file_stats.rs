use std::fs;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::language::Language;
use crate::language::Language::OTHER;

#[derive(Clone, Debug, Copy)]
pub struct FileStats {
    pub code_lines: u16,
    pub empty_lines: u16,
    pub comment_lines: u16,
}

impl FileStats {
    /// Counts stats in a file and adds them.
    ///
    /// When the file format doesn't match or the  no stats are added and false is returned.
    pub fn new(path: &str, lang: &Language) -> Self {
        let mut inst = FileStats { code_lines: 0, empty_lines: 0, comment_lines: 0 };
        match lang {
            Language::JAVA => inst.analyze_java(path),
            Language::KOTLIN => inst.analyze_java(path),
            Language::C => inst.analyze_java(path),
            Language::CPP => inst.analyze_java(path),
            Language::RUST => inst.analyze_java(path), // FIXME: ignoring in file tests
            Language::PYTHON => inst.analyze_bash(path), // FIXME: ignoring multiline string comments and tests
            Language::GRADLE => inst.analyze_java(path),
            Language::CMAKE => inst.analyze_bash(path), // https://cmake.org/cmake/help/v3.1/manual/cmake-language.7.html#comments
            Language::MAKEFILE => inst.analyze_bash(path),
            Language::ASSEMBLY => inst.analyze_general(path, CommentStyle::UNKNOWN), // TODO: implement
            OTHER(_) => inst.analyze_general(path, CommentStyle::UNKNOWN),
        }
        inst
    }

    /// Analyze a undifferentiated file.
    fn analyze_general(&mut self, path: &str, comments: CommentStyle) {
        let lines = count_differentiated_lines(&path, comments);
        self.util_add_lines(lines);
    }

    fn analyze_java(&mut self, path: &str) {
        self.analyze_general(path, CommentStyle::C)
    }

    fn analyze_bash(&mut self, path: &str) {
        let lines = count_differentiated_lines(path, CommentStyle::BASH);
        self.util_add_lines(lines);
    }

    fn util_add_lines(&mut self, lines: Option<(usize, usize, usize)>) {
        if let Some(lines) = lines {
            self.code_lines += (lines.0 - (lines.1 + lines.2)) as u16;
            self.comment_lines += lines.1 as u16;
            self.empty_lines += lines.2 as u16;
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