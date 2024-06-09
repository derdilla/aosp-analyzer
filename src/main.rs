use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use peak_alloc::PeakAlloc;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::counter::{CountContext, HasStats};
use crate::language::Language;

mod counter;
mod language;
mod lang_stats;
mod file_stats;

#[cfg(debug_assertions)]
#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

//const ANDROID_SOURCE: &str = "/home/derdilla/android-source/aosp14/";
const ANDROID_SOURCE: &str = "/home/derdilla/Coding/Java";

fn main() {
    /*
    let scan = Scanner::scan(ANDROID_SOURCE);
    let mut context = CountContext::new();
    for f in scan.files {
        context.insert_file(f.unwrap());
    }

    println!("{:#?}", context.stats());

     */
    let start = SystemTime::now();

    let context = CountContext::new(ANDROID_SOURCE.to_string());
    let context = scan_dir(ANDROID_SOURCE, context);
    //println!("{:#?}", context);
    println!("Analyzing {ANDROID_SOURCE} took: {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
    // 511ms -> 941ms -> 524ms -> 509ms -> 477ms

    if let Ok(mut file) = File::create("analysis_context.txt") {
        file.write_all(serde_json::to_string(&context).unwrap().as_bytes()).unwrap()
    }

    let stats = context.stats();
    if let Ok(mut file) = File::create("analysis_stats.txt") {
        file.write_all(serde_json::to_string(&stats).unwrap().as_bytes()).unwrap()
    }

    #[cfg(debug_assertions)]
    println!("{}", print_hierarchy(&context, 0));

    if cfg!(debug_assertions) {
        let mut map = HashMap::<Language, u32>::new();
        count_extensions(&context, &mut map);
        println!("Files: {:#?}", map);
    }

    if cfg!(debug_assertions) {
        println!("The max amount that was used {}GB", PEAK_ALLOC.peak_usage_as_gb());
        println!("The currently used amount is {}GB", PEAK_ALLOC.current_usage_as_gb());
    }
}

fn scan_dir<P: AsRef<Path>>(dir: P, mut context: CountContext) -> CountContext {
    let data = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .filter(|path| !(path.ends_with(".repo")
            || path.ends_with(".git")
            || path.ends_with("prebuilt")
            || path.ends_with("prebuilts")
            || path.ends_with("out"))
        )
        .map(|entry: &PathBuf| {
            if entry.is_dir() && entry.read_dir().unwrap().next().is_some() {
                scan_dir(entry, CountContext::new(entry.file_name().unwrap().to_str().unwrap().to_string()))
            } else if entry.is_file() {
                let mut context = CountContext::new(entry.file_name().unwrap().to_str().unwrap().to_string());
                if !["jar", "so", "obj", "webp", "class", "jpeg", "exe", "webm",
                    "mp4", "apk", "apex", "ko", "lz4", "gz", "debug", "cr2",
                ].iter().any(|ext| entry.ends_with(ext)) {
                    context.insert_file(&entry);
                }
                context
            } else {
                CountContext::new(String::new())
            }
        })
        .filter(|e| !e.is_empty())
        .collect::<Vec<CountContext>>();
    for e in data {
        context.insert_context(e);
    }
    context
}

fn print_hierarchy(context: &CountContext, level: usize) -> String {
    let mut str = context.name() + "\n";
    {
        let mut dirs_iter = context.dirs.iter().peekable();
        while dirs_iter.peek().is_some() {
            let val = print_hierarchy(&dirs_iter.next().unwrap(), level + 1);
            if dirs_iter.peek().is_some()
                || !context.files.is_empty() {
                str += ("│".repeat(level) + "├ " + val.as_str()).as_str();
            } else {
                str += ("│".repeat(level) + "└ " + val.as_str()).as_str();
            }

        }
    }

    {
        let mut file_iter = context.files.iter().peekable();
        while file_iter.peek().is_some() {
            let val = &file_iter.next().unwrap().name();
            if file_iter.peek().is_some() {
                str += ("│".repeat(level) + "├ " + val.as_str() + "\n").as_str();
            } else {
                str += ("│".repeat(level) + "└ " + val.as_str() + "\n").as_str();
            }

        }
    }
    str // TODO: test
}

/// HashMap::<Language, u32>::new()
fn count_extensions(context: &CountContext, map: &mut HashMap::<Language, u32>) {
    for dir in &context.dirs {
        count_extensions(dir, map);
    }
    for file in &context.files {
        let mut entry = map.entry(file.lang.clone()).or_insert(0);
        *entry += 1;
    }
}

