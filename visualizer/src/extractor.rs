use std::collections::HashMap;

use crate::parser::{TokeiCodeStatistics, Data};

pub struct ExtractedData {
    pub core: CodeCategory,
    pub devtools: CodeCategory,
    pub sdks: CodeCategory,
    pub thirdparty: CodeCategory,
    pub userspace: CodeCategory,
    pub tests: CodeCategory
}

impl ExtractedData {
    pub fn new(mut data: Data) -> Self {
        let mut tests_details = HashMap::new();
        
        let core = Self::extract("Core", data.core.remove("Total").unwrap(), &mut tests_details);
        let devtools = Self::extract("Devtools", data.devtools.remove("Total").unwrap(), &mut tests_details);
        let sdks = Self::extract("SDKs", data.sdks.remove("Total").unwrap(), &mut tests_details);
        let thirdparty = Self::extract("Third-party", data.thirdparty.remove("Total").unwrap(), &mut tests_details);
        let userspace = Self::extract("Userspace", data.userspace.remove("Total").unwrap(), &mut tests_details);

        let tests_total = {
            let mut s = CodeStats::default();
            for (_, stats) in tests_details.clone() {
                s.code += stats.code;
                s.comment += stats.comment;
                s.blank += stats.blank;
            }
            s
        };
        let tests = CodeCategory{
            name: String::from("Tests"),
            details: tests_details.into_iter().collect::<Vec<(String, CodeStats)>>(),
            total: tests_total,
        };

        ExtractedData {
            core,
            devtools,
            sdks,
            thirdparty,
            userspace,
            tests,
        }
    }

    fn extract(name: &str, stats: TokeiCodeStatistics, tests: &mut HashMap<Language, CodeStats>) -> CodeCategory {
        let mut res = CodeCategory::new(name.to_string());
        for (lang, files) in stats.children.unwrap() {
            let mut lang_stats = CodeStats::default();
            for report in files {
                if report.name.contains("test") {
                    let entry = tests.entry(lang.clone()).or_insert(CodeStats::default());
                    entry.code += report.stats.code;
                    entry.comment += report.stats.comments;
                    entry.blank += report.stats.blanks;
                } else {
                    //println!("Prod: {}", report.name);
                    lang_stats.code += report.stats.code;
                    lang_stats.comment += report.stats.comments;
                    lang_stats.blank += report.stats.blanks;
                }
            }
            res.total.code += lang_stats.code;
            res.total.comment += lang_stats.comment;
            res.total.blank += lang_stats.blank;
            res.details.push((lang, lang_stats));
        }

        res
    }
}

type Language = String;

pub struct CodeCategory {
    pub name: String,
    pub total: CodeStats,
    pub details: Vec<(Language, CodeStats)>,
}

#[derive(Clone)]
pub struct CodeStats {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
}


impl Default for CodeStats {
    fn default() -> Self {
        Self { code: 0, comment: 0, blank: 0 }
    }
}

impl CodeCategory {
    fn new(name: String) -> Self {
        Self { name, total: CodeStats::default(), details: Vec::new() }
    }
}
