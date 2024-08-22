use std::{collections::HashMap, fmt::format};

use crate::extractor::*;

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

pub fn generate_html(data: &ExtractedData) -> String {
    let overview = build_overview(&data);
    let details = build_details(data);
    let out = format!("<!DOCTYPE html><html><head>{STYLE}</head><body>{overview}{details}</body></html>");
    return out;
}

fn build_details(data: &ExtractedData) -> String {
    let mut out = String::from("<div class=\"tabs\">");
    out += build_lang_detail(&data.core).as_str();
    out += build_lang_detail(&data.sdks).as_str();
    out += build_lang_detail(&data.thirdparty).as_str();
    out += build_lang_detail(&data.devtools).as_str();
    out += build_lang_detail(&data.userspace).as_str();
    out += build_lang_detail(&data.tests).as_str();
    out += build_lang_detail(&data.docs).as_str();
    out += "</div>";
    format!("<section><h2>Details</h2>{out}</section>")
}

fn build_lang_detail(data: &CodeCategory) -> String {
    let mut details = data.details.clone();
    details.sort_by(|(_, stats_a), (_, stats_b)| stats_b.code.cmp(&stats_a.code));

    let t_head = format!("<thead><tr>{}{}{}{}{}",
        "<th scope=\"col\"></th>",
        "<th scope=\"col\">Code</th>",
        "<th scope=\"col\">Comments</th>",
        "<th scope=\"col\">Blank</th>",
    "</tr></thead>");
    let input = format!("<input type=\"radio\" id=\"tab-{}\" name=\"tab-group-1\">", &data.name);
    let label = format!("<label for=\"tab-{}\">{}</label>", &data.name, &data.name);

    let mut out = String::new();
    for (lang, v) in details {
        // TODO: exclude 0 lines

        out += format!("<tr><th scope=\"row\">{lang}</th><td>{}</td><td>{}</td><td>{}</td></tr>",  v.code, v.comment, v.blank, ).as_str();
    }
    let out = format!("<tbody>{out}</tbody>");
    let out = format!("<table>{t_head}{out}</table>");
    let out = format!("<div class=\"content\">{out}</div>");
    let out = format!("<div class=\"tab\">{input}{label}{out}</div>");
    // FIXME: make like html below

    out
}

fn build_overview(data: &ExtractedData) -> String {
    let total_code = data.core.total.code
        + data.sdks.total.code
        + data.thirdparty.total.code
        + data.devtools.total.code
        + data.userspace.total.code;
    let total_comments = data.core.total.comment
        + data.sdks.total.comment
        + data.thirdparty.total.comment
        + data.devtools.total.comment
        + data.userspace.total.comment;
    let total_blank = data.core.total.blank
        + data.sdks.total.blank
        + data.thirdparty.total.blank
        + data.devtools.total.blank
        + data.userspace.total.blank;
    let total = total_code + total_comments + total_blank;

    let running = data.core.total.code
        + data.sdks.total.code
        + data.thirdparty.total.code
        + data.userspace.total.code;

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