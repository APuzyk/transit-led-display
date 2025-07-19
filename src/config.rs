use rpi_led_matrix::LedColor;
use std::collections::HashMap;
use yaml_rust2::Yaml;

#[derive(Clone)]
pub struct ColorConfig {
    pub top_line_color: LedColor,
    pub line_name_color: LedColor,
    pub tta_color: LedColor,
    pub standard_color: LedColor,
    pub rapid_line_color: LedColor,
    pub no_location_color: LedColor,
}

impl ColorConfig {
    pub fn new(config: &Yaml) -> Self {
        Self {
            top_line_color: LedColor {
                red: config["top_line_color"]["red"].as_i64().unwrap() as u8,
                green: config["top_line_color"]["green"].as_i64().unwrap() as u8,
                blue: config["top_line_color"]["blue"].as_i64().unwrap() as u8,
            },
            line_name_color: LedColor {
                red: config["line_name_color"]["red"].as_i64().unwrap() as u8,
                green: config["line_name_color"]["green"].as_i64().unwrap() as u8,
                blue: config["line_name_color"]["blue"].as_i64().unwrap() as u8,
            },
            tta_color: LedColor {
                red: config["tta_color"]["red"].as_i64().unwrap() as u8,
                green: config["tta_color"]["green"].as_i64().unwrap() as u8,
                blue: config["tta_color"]["blue"].as_i64().unwrap() as u8,
            },
            standard_color: LedColor {
                red: config["standard_color"]["red"].as_i64().unwrap() as u8,
                green: config["standard_color"]["green"].as_i64().unwrap() as u8,
                blue: config["standard_color"]["blue"].as_i64().unwrap() as u8,
            },
            rapid_line_color: LedColor {
                red: config["rapid_line_color"]["red"].as_i64().unwrap() as u8,
                green: config["rapid_line_color"]["green"].as_i64().unwrap() as u8,
                blue: config["rapid_line_color"]["blue"].as_i64().unwrap() as u8,
            },
            no_location_color: LedColor {
                red: config["no_location_color"]["red"].as_i64().unwrap() as u8,
                green: config["no_location_color"]["green"].as_i64().unwrap() as u8,
                blue: config["no_location_color"]["blue"].as_i64().unwrap() as u8,
            },
        }
    }
    pub fn top_line_color(&self) -> &LedColor {
        &self.top_line_color
    }
    pub fn line_name_color(&self) -> &LedColor {
        &self.line_name_color
    }
    pub fn tta_color(&self) -> &LedColor {
        &self.tta_color
    }
    pub fn standard_color(&self) -> &LedColor {
        &self.standard_color
    }
    pub fn rapid_line_color(&self) -> &LedColor {
        &self.rapid_line_color
    }
    pub fn no_location_color(&self) -> &LedColor {
        &self.no_location_color
    }
}
pub struct DisplayBoardConfig {
    font_file: String,
    rows: u32,
    cols: u32,
    chained: u32,
    line_ref_to_display_position: HashMap<String, (i32, i32)>,
    color_config: ColorConfig,
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
            color_config: ColorConfig::new(&config["color_config"]),
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
    pub fn color_config(&self) -> &ColorConfig {
        &self.color_config
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
