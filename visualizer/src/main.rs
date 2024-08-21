use std::{collections::HashMap, fs};

use format::generate_html;
use parser::Data;

mod parser;
mod format;

fn main() {
    let data = Data::load("../stats.sample/");
    let out = generate_html(&data);
    fs::write("tst.html", out);
}
