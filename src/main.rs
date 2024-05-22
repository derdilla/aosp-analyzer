use std::fs;
use std::fs::{DirEntry, FileType};
use std::path::Path;
use std::time::SystemTime;
use crate::counter::{CountContext, HasStats, SourceFile};
use crate::scanner::Scanner;

mod counter;
mod language;
mod lang_stats;
mod scanner;

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

    let mut context = CountContext::new();
    scan_dir(ANDROID_SOURCE, &mut context);
    // println!("{:#?}", context);
    println!("Analyzing {ANDROID_SOURCE} took: {}s", SystemTime::now().duration_since(start).unwrap().as_secs())
    // 20s

    // TODO: async & multithread reading
}

fn scan_dir<P: AsRef<Path>>(dir: P, context: &mut CountContext) {
    // TODO: Scanner filtering

    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(entry) = entry {
            let entry_type = entry.file_type().unwrap();
            if entry_type.is_file() {
                context.insert_file(entry);
            } else if entry_type.is_dir() {
                let mut inner_context = CountContext::new();
                scan_dir(entry.path(), &mut inner_context);
                context.insert_context(inner_context);
            }
        }
    }
}

