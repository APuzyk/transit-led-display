use std::collections::HashMap;
use yaml_rust2::Yaml;

pub struct DisplayBoardConfig {
    font_file: String,
    rows: u32,
    cols: u32,
    chained: u32,
    line_ref_to_display_position: HashMap<String, (i32, i32)>,
}
impl DisplayBoardConfig {
    pub fn new(config: &Yaml) -> Self {
        Self {
            font_file: config["font_file"].as_str().unwrap().to_string(),
            rows: config["rows"].as_i64().unwrap() as u32,
            cols: config["cols"].as_i64().unwrap() as u32,
            chained: config["chained"].as_i64().unwrap() as u32,
            line_ref_to_display_position: config["line_ref_to_display_position"]
                .as_hash()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str().unwrap().to_string(),
                        (
                            v["x"].as_i64().unwrap() as i32,
                            v["y"].as_i64().unwrap() as i32,
                        ),
                    )
                })
                .collect(),
        }
    }
    pub fn font_file(&self) -> &str {
        &self.font_file
    }
    pub fn rows(&self) -> u32 {
        self.rows
    }
    pub fn cols(&self) -> u32 {
        self.cols
    }
    pub fn chained(&self) -> u32 {
        self.chained
    }
    pub fn line_ref_to_display_position(&self) -> &HashMap<String, (i32, i32)> {
        &self.line_ref_to_display_position
    }
}

pub struct Config {
    display_board_config: DisplayBoardConfig,
    rapid_line_to_parent_line_map: HashMap<String, String>,
    stops_to_monitor: Vec<String>,
}

impl Config {
    pub fn new(config: &Yaml) -> Self {
        Self {
            display_board_config: DisplayBoardConfig::new(&config["display_board_config"]),
            rapid_line_to_parent_line_map: config["rapid_line_to_parent_line_map"]
                .as_hash()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    (
                        k.as_str().unwrap().to_string(),
                        v.as_str().unwrap().to_string(),
                    )
                })
                .collect(),
            stops_to_monitor: config["stops_to_monitor"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
        }
    }
    pub fn display_board_config(&self) -> &DisplayBoardConfig {
        &self.display_board_config
    }
    pub fn rapid_line_to_parent_line_map(&self) -> &HashMap<String, String> {
        &self.rapid_line_to_parent_line_map
    }
    pub fn stops_to_monitor(&self) -> &Vec<String> {
        &self.stops_to_monitor
    }
}
