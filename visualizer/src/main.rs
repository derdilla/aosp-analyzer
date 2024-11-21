use std::{collections::HashMap, fs};

use extractor::ExtractedData;
use format::generate_html;
use parser::Data;

mod parser;
mod format;
mod extractor;
mod outputs;

fn main() {
    let data = Data::load("../stats/");
    let core_lange_percent = outputs::lang_percent::create(data.lang_overview());

    

    fs::write("core_lange_percent.json", core_lange_percent).unwrap();
    //let data = ExtractedData::new(data);
    //let out = generate_html(&data);
    //fs::write("index.html", out);
}
