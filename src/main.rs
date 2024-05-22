use crate::counter::CountContext;
use crate::scanner::Scanner;

mod counter;
mod language;
mod lang_stats;
mod scanner;

//const ANDROID_SOURCE: &str = "/home/derdilla/android-source/aosp14/";
const ANDROID_SOURCE: &str = "/home/derdilla/Coding/Java";

fn main() {
    let scan = Scanner::scan(ANDROID_SOURCE);
    let mut context = CountContext::new();
    for f in scan.files {
        context.insert(f.unwrap());
    }

    // TODO: split up directories

    println!("{:#?}", context);


    // TODO: async reading
}