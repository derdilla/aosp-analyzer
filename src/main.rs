use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use peak_alloc::PeakAlloc;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::counter::{CountContext, HasStats};

mod counter;
mod language;
mod lang_stats;
mod file_stats;

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

    let peak_mem = PEAK_ALLOC.peak_usage_as_gb();
    println!("The max amount that was used {}GB", peak_mem);
    // 0.39365256GB -> 0.34015447GB -> 0.3339298GB -> 0.34041083GB -> 0.30123457GB
    // -> 0.33434874GB
    println!("The currently used amount is {}GB", PEAK_ALLOC.current_usage_as_gb());
    // 0.01678145GB -> 0.016183667GB -> 0.022987688GB -> 0.02162337GB -> 0.014041128GB
    // -> 0.010378797GB

    let start = SystemTime::now();
    context.stats();
    println!("Building stats took: {}s", SystemTime::now().duration_since(start).unwrap().as_secs());

    //println!("{}", print_hierarchy(&context, 0));
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
    for e in &context.children {
        if let Some(context) = e.context() {
            str += (" ".repeat(level) + "├ " + print_hierarchy(context, level + 1).as_str()).as_str();
        } else {
            str += (" ".repeat(level) + "├ " + e.name().as_str() + "\n").as_str();
        }
    }
    str
}

