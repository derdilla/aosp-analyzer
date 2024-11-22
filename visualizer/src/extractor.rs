use std::collections::HashMap;

use crate::parser::{TokeiCodeStatistics, Data};

pub struct ExtractedData {
    pub core: CodeCategory,
    pub devtools: CodeCategory,
    pub sdks: CodeCategory,
    pub thirdparty: CodeCategory,
    pub userspace: CodeCategory,
    pub tests: CodeCategory,
    pub docs: CodeCategory,
}

impl ExtractedData {
    pub fn new(mut data: Data) -> Self {
        let mut tests_details = HashMap::new();
        // Documentation only files.
        let mut doc_details = HashMap::new();
        
        let core = Self::extract("Core", data.core.remove("Total").unwrap(), &mut tests_details, &mut doc_details);
        let devtools = Self::extract("Devtools", data.devtools.remove("Total").unwrap(), &mut tests_details, &mut doc_details);
        let sdks = Self::extract("SDKs", data.sdks.remove("Total").unwrap(), &mut tests_details, &mut doc_details);
        let thirdparty = Self::extract("Third-party", data.thirdparty.remove("Total").unwrap(), &mut tests_details, &mut doc_details);
        let userspace = Self::extract("Userspace", data.userspace.remove("Total").unwrap(), &mut tests_details, &mut doc_details);

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

        let docs_total = {
            let mut s = CodeStats::default();
            for (_, stats) in doc_details.clone() {
                s.code += stats.code;
                s.comment += stats.comment;
                s.blank += stats.blank;
            }
            s
        };
        let docs = CodeCategory{
            name: String::from("Docs"),
            details: doc_details.into_iter().collect::<Vec<(String, CodeStats)>>(),
            total: docs_total,
        };

        ExtractedData {
            core,
            devtools,
            sdks,
            thirdparty,
            userspace,
            tests,
            docs,
        }
    }

    fn extract(name: &str, stats: TokeiCodeStatistics, tests: &mut HashMap<Language, CodeStats>, docs: &mut HashMap<Language, CodeStats>) -> CodeCategory {
        let mut res = CodeCategory::new(name.to_string());
        for (lang, files) in stats.children.unwrap() {
            if lang == "Json" || lang == "Xml" || lang == "Yaml" || lang == "Toml" || lang == "Ini" {
                continue; // Skip is data entries
            }
            let mut lang_stats = CodeStats::default();
            for report in files {
                let entry = if report.name.contains("test") {
                    tests.entry(lang.clone()).or_insert(CodeStats::default())
                } else if report.name.contains("/doc/") || report.name.contains("/docs/") {
                    docs.entry(lang.clone()).or_insert(CodeStats::default())
                } else {
                    //println!("Prod: {}", report.name);
                    lang_stats.code += report.stats.code;
                    lang_stats.comment += report.stats.comments;
                    lang_stats.blank += report.stats.blanks;
                    continue;
                };
                // data could be client side and interactive
                entry.code += report.stats.code;
                entry.comment += report.stats.comments;
                entry.blank += report.stats.blanks;
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
