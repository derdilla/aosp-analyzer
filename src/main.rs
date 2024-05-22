use std::fs;
use std::fs::{DirEntry, FileType};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
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

    let context = CountContext::new();
    let context = scan_dir(ANDROID_SOURCE, context);
    // println!("{:#?}", context);
    println!("Analyzing {ANDROID_SOURCE} took: {}s", SystemTime::now().duration_since(start).unwrap().as_secs())
    // Multithreading is faster: 20s -> 3s
}

fn scan_dir<P: AsRef<Path>>(dir: P, mut context: CountContext) -> CountContext {
    // TODO: Scanner filtering
    let data = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .map(|entry: &PathBuf| {
            if entry.is_dir() {
                scan_dir(entry, CountContext::new())
            } else if entry.is_file() {
                let mut context = CountContext::new();
                context.insert_file(&entry);
                context
            } else {
                CountContext::new()
            }
        })
        .collect::<Vec<CountContext>>();
    for e in data {
        context.insert_context(e);
    }
    context
}

