use std::{collections::HashMap, fs};

use extractor::ExtractedData;
use format::generate_html;
use parser::Data;

mod parser;
mod format;
mod extractor;

fn main() {
    let data = Data::load("../stats.sample/");
    let data = ExtractedData::new(data);
    let out = generate_html(&data);
    fs::write("tst.html", out);
}
