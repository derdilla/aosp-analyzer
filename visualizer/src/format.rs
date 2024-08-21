use std::{collections::HashMap, fmt::format};

use crate::parser::*;

const STYLE: &'static str = "<style>
.tabs {
    --bg: #262626;
    background: var(--bg);
    color: white;
    display: block;
    position: relative;
    min-height: 45em;
    width: max-content;
    clear: both;
    margin: 25px 0;
    border-radius: 5px;
}
.tab {
    float: left;
    padding-top: 10px;
    padding-right: 1px;
}
.tab label {
    padding: 10px;
    margin-left: -1px;
    position: relative;
    left: 1px; 
    border-radius: 5px;
    /* padding-inline: 20px; */
}
.tab [type=radio] {
    display: none;
}
.content {
    position: absolute;
    top: 28px;
    left: 0;
    right: 0;
    bottom: 0;
    padding: 20px;
    background: var(--bg);
    height: 40em;
    scroll-behavior: smooth;
    overflow: scroll;
}
[type=radio]:checked ~ label {
    background-color: #56B7FD;
    color: #000;
    border-radius: 2px;
    z-index: 2;
}
[type=radio]:checked ~ label ~ .content {
    z-index: 1;
}
</style>";

pub fn generate_html(data: &Data) -> String {
    let overview = build_overview(&data);
    let details = build_details(data);
    let out = format!("<!DOCTYPE html><html><head>{STYLE}</head><body>{overview}{details}</body></html>");
    return out;
}

fn build_details(data: &Data) -> String {
    let mut out = String::from("<div class=\"tabs\">");
    out += build_lang_detail("Core", &data.core).as_str();
    out += build_lang_detail("Sdks", &data.sdks).as_str();
    out += build_lang_detail("Thirdparty code", &data.thirdparty).as_str();
    out += build_lang_detail("Devtools", &data.devtools).as_str();
    out += build_lang_detail("Userspace", &data.userspace).as_str();
    out += "</div>";
    out
}

fn build_lang_detail(title: &str, data: &HashMap<String, CodeStatistics>) -> String {
    let mut data = data.iter()
        //.map(|e| e.0.clone())
        .collect::<Vec<(&String, &CodeStatistics)>>();
    data.sort_by(|a, b| b.1.code.cmp(&a.1.code));

    let t_head = format!("<thead><tr>{}{}{}{}{}",
        "<th scope=\"col\"></th>",
        "<th scope=\"col\">Code</th>",
        "<th scope=\"col\">Comments</th>",
        "<th scope=\"col\">Blank</th>",
    "</tr></thead>");
    let input = format!("<input type=\"radio\" id=\"tab-{}\" name=\"tab-group-1\">", &title);
    let label = format!("<label for=\"tab-{}\">{}</label>", &title, &title);

    let mut out = String::new();
    for (lang, v) in data {
        out += format!("<tr><th scope=\"row\">{lang}</th><td>{}</td><td>{}</td><td>{}</td></tr>",  v.code, v.comments, v.blanks, ).as_str();
    }
    let out = format!("<tbody>{out}</tbody>");
    let out = format!("<table>{t_head}{out}</table>");
    let out = format!("<div class=\"content\">{out}</div>");
    let out = format!("<div class=\"tab\">{input}{label}{out}</div>");
    // FIXME: make like html below

    format!("<section><h2>Details</h2>{out}</section>")
}

fn build_overview(data: &Data) -> String {
    let total_code = data.core.get("Total").unwrap().code
        + data.sdks.get("Total").unwrap().code
        + data.thirdparty.get("Total").unwrap().code
        + data.devtools.get("Total").unwrap().code
        + data.userspace.get("Total").unwrap().code;
    let total_comments = data.core.get("Total").unwrap().comments
        + data.sdks.get("Total").unwrap().comments
        + data.thirdparty.get("Total").unwrap().comments
        + data.devtools.get("Total").unwrap().comments
        + data.userspace.get("Total").unwrap().comments;
    let total_blank = data.core.get("Total").unwrap().blanks
        + data.sdks.get("Total").unwrap().blanks
        + data.thirdparty.get("Total").unwrap().blanks
        + data.devtools.get("Total").unwrap().blanks
        + data.userspace.get("Total").unwrap().blanks;
    let total = total_code + total_comments + total_blank;

    let running = data.core.get("Total").unwrap().code
        + data.sdks.get("Total").unwrap().code
        + data.thirdparty.get("Total").unwrap().code
        + data.userspace.get("Total").unwrap().code;

    let msg_total = format!("There are over <b>{}</b> source code lines contributing to android.", build_number(total));
    let msg_running = format!("Roughly <b>{}</b> lines of code run on the average device.", build_number(running));
    let msg_doc = format!("<b>{}</b> lines of comments tell the developers what the code does.", build_number(total_comments));
    let msg_empty = format!("<b>{:.1}%</b> lines are empty.", (total_blank as f64 / total  as f64) * 100.);

    // TODO: doughnut chart with language percentages
    
    let list = format!("<ul><li>{}</li><li>{}</li><li>{}</li><li>{}</li></ul>", msg_total, msg_running, msg_doc, msg_empty);
    format!("<section><h2>Quick facts</h2>{list}</section>")
    
}

fn build_number(num: u32) -> String {
    if num < 1000 {
        num.to_string()
    } else if num < 1000_000 {
        format!("{} thousand", num / 1000)
    }
    else {
        format!("{} million", num / 1000_000)
    }
}