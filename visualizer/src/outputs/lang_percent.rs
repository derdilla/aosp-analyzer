use std::collections::HashMap;

use serde::Serialize;

use crate::parser::TokeiCodeStatistics;

/// Minimum percent to show as own category in the graph.
const MIN_PERCENT: f32 = 0.02;

#[derive(Debug, Serialize)]
struct Lang {
    category: String,
    #[serde(rename = "value")]
    percent: f32,
    #[serde(rename = "loc")]
    lines_of_code: u64,
}


/// Create data for a doughnut chart containing all languges percentages above [MIN_PERCENT] and "Other" in json.
pub fn create_stat(data: HashMap<String, TokeiCodeStatistics>) -> String {
    let total_lines = data.values().fold(0u64, |acc, x| acc + x.code as u64);
    
    // accumulate stats
    let mut stats = Vec::new();
    let mut other: f32 = 0.0;
    let mut other_lines: u64 = 0;
    for (category, lines) in data {
        let percent = (lines.code as f64 / total_lines as f64) as f32;
        if percent > MIN_PERCENT {
            stats.push(Lang{category, percent, lines_of_code: lines.code as u64});
        } else {
            println!("{}: {}\t{}/{}", &category, &percent,&lines.code,&total_lines);
            other += percent as f32;
            other_lines += lines.code as u64;
        }
    }
    
    // sort
    stats.sort_by(|a, b| a.percent.partial_cmp(&b.percent).expect("should not contain NaN values"));

    // add "Other" category to the end
    stats.push(Lang {category: String::from("Other"), percent: other, lines_of_code: other_lines});

    serde_json::to_string(&stats).expect("Data created to be jsonizable")
}

/// Create a vega json for a doughnut chart containing all languges percentages above [MIN_PERCENT] and "Other".
pub fn create(data: HashMap<String, TokeiCodeStatistics>) -> String {
    GRAPHIC_TEMPLATE.replace("{{ DATA }}", create_stat(data).as_str())
}

/// Template for vega graphic.
/// 
/// `{{ DATA }}` should be replaced with something like:
/// ````json
/// [
///   { "category": "Java", "value": 4 },
///   { "category": "C", "value": 6 },
/// ]
/// ```
const GRAPHIC_TEMPLATE: &'static str = r#"{
    "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
    "description": "A simple donut chart with embedded data.",
    "data": {
        "values": {{ DATA }}
    },
    "layer": [
        {
            "params": [
                {
                    "name": "highlight",
                    "select": { "type": "point", "on": "pointerover" }
                },
            ],
            "mark": {
                "type": "arc",
                "innerRadius": 50,
                "outerRadius": 100,
                "cornerRadius": 6,
                "padAngle": 0.04
            },
            "encoding": {
                "color": { "field": "category", "type": "nominal", "legend": null },
                "opacity": {
                    "condition": [
                        {
                            "param": "highlight",
                            "value": 1
                        }
                    ],
                    "value": 0.70
                },
                "tooltip": [
                    { "field": "value", "type": "quantitative", "title": "%" }
                ]
            },
        },
        {
            "mark": { "type": "text", "radius": 75 },
            "encoding": {
                "text": { "field": "category", "type": "nominal" }
            }
        },
        {
            "mark": {
                "type": "text",
                "text": "Lanugages"
            },

        }
    ],
    "encoding": {
        "theta": { "field": "value", "type": "quantitative", "stack": true }
    }
}"#;
