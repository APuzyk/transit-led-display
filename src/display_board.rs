use chrono::{DateTime, Datelike, Local};
use std::collections::HashMap;
use std::path::Path;
use std::{thread, time};

use crate::stop_monitor::MonitoredVehicleJourney;
use log::debug;
use rpi_led_matrix::{LedColor, LedFont, LedMatrix, LedMatrixOptions, LedCanvas};

// CONSTANTS for display

// top line purplihs
const TOP_LINE_COLOR: LedColor = LedColor {
    red: 0,
    green: 0,
    blue: 150,
};

// Line name color
const LINE_NAME_COLOR: LedColor = LedColor {
    red: 255,
    green: 255,
    blue: 20,
};

// Time to arrival color
const TTA_COLOR: LedColor = LedColor {
    red: 255,
    green: 140,
    blue: 0,
};
// Used for most text on teh board
const STANDARD_COLOR: LedColor = LedColor {
    red: 0,
    green: 127,
    blue: 255,
};

// legacy: used to distinguish rapid line times when we aggregate line time
const RAPID_LINE_COLOR: LedColor = LedColor {
    red: 255,
    green: 0,
    blue: 0,
};

// COlor of the dot used to indicate that there is no locaiton
const NO_LOC_COLOR: LedColor = LedColor {
    red: 255,
    green: 0,
    blue: 0,
};

// Number of column used for Line Refs (e.g. 9, 22, 14R)
const LINE_REF_N_CHARS: usize = 4;
const LINE_REF_BUFFER_COLS: i32 = 6;
// Number of chars to use for time to arrivals
const TTA_N_CHARS: usize = 2;
// Number of led pixel width to provvide for TTA
const TTA_BUFFER_COLS: i32 = 4;

// COL WIDTH OF A SINGLE BOARD
const COL_WIDTH: i32 = 64;

const FONT_HEIGHT: i32 = 6;

pub struct DisplayBoard {
    pub display_lines: Option<HashMap<String, Vec<MonitoredVehicleJourney>>>,
    pub last_successful_request_time: Option<DateTime<Local>>,
    pub last_request_successful: bool,
    pub led_matrix: LedMatrix,
    pub led_canvas: LedCanvas,
    pub font: LedFont,
    pub display_position_map: HashMap<String, (i32, i32)>,
}

pub struct RGBDisplayLine {
    line: Vec<LineString>,
}

impl RGBDisplayLine {
    pub fn new() -> Self {
        RGBDisplayLine { line: Vec::new() }
    }
}

pub struct LineString {
    string: String,
    color: LedColor,
    has_loc: bool,
    is_line_ref: bool,
}

impl LineString {
    pub fn new() -> Self {
        LineString {
            string: "".to_string(),
            color: STANDARD_COLOR,
            has_loc: false,
            is_line_ref: false,
        }
    }
}

impl DisplayBoard {
    pub fn new(
        rows: u32,
        cols: u32,
        chained: u32,
        font_file: &Path,
        display_position_map: &HashMap<String, (i32, i32)>,
    ) -> Result<Self, &'static str> {
        let mut options = LedMatrixOptions::new();
        debug!("Setting rows to {}", rows);
        debug!("Setting cols to {}", cols);
        options.set_rows(rows);
        options.set_cols(cols);
        debug!("Setting chain length to {}", chained);
        options.set_chain_length(2);
        options.set_hardware_mapping("adafruit-hat");

        let led_matrix = LedMatrix::new(Some(options), None)?;
        debug!("creating canvas");
        let mut led_canvas = led_matrix.canvas();
        debug!("loading font from {:?}", font_file);
        let font = LedFont::new(font_file)?;

        let mut d = DisplayBoard {
            display_lines: None,
            last_successful_request_time: None,
            last_request_successful: false,
            led_matrix: led_matrix,
            led_canvas: led_canvas,
            font: font,
            display_position_map: display_position_map.clone(),
        };
        Ok(d)
    }

    pub fn test_write(&mut self) {
        // let mut canvas = self.led_matrix.offscreen_canvas();

        // Your vertical line
        for y in 10..=17 {
            self.led_canvas.set(10, y, &STANDARD_COLOR);
        }

        // Get font metrics if available
        let (height, width) = self.led_canvas.canvas_size();
        debug!("Canvas dimensions: {}x{}", width, height);

        // Try different Y positions - BDF fonts often have baseline issues
        let test_positions = vec![
            (0, 0, "Top-left"),
            (0, 8, "Y=8"),
            (0, 16, "Y=16"),
            (0, 24, "Y=24"),
            (0, height as i32 - 1, "Bottom"),
        ];

        for (x, y, label) in test_positions {
            debug!("Drawing '{}' at ({}, {})", label, x, y);
            self.led_canvas.draw_text(&self.font, label, x, y, &STANDARD_COLOR, 0, false);
        }
        self.led_canvas.draw_text(&self.font, "Hello", 2, 2, &STANDARD_COLOR, 0, false);

        // canvas = self.led_matrix.swap(canvas);
    }

    pub fn test_color(&mut self, red: u8, green: u8, blue: u8) {
        // let mut canvas = self.led_matrix.offscreen_canvas();
        let text = format!("r {red}, g: {green}, b: {blue}");
        let color = LedColor {
            red: red,
            green: green,
            blue: blue,
        };
        self.led_canvas.draw_text(&self.font, &format!("red {red}"), 2, 12, &color, 0, false);
        self.led_canvas.draw_text(
            &self.font,
            &format!("green: {green}"),
            2,
            18,
            &color,
            0,
            false,
        );
        self.led_canvas.draw_text(
            &self.font,
            &format!("blue: {blue}"),
            2,
            24,
            &color,
            0,
            false,
        );
        for y in 10..22 {
            self.led_canvas.draw_line(80, y, 96, y, &color);
        }
        // canvas = self.led_matrix.swap(canvas);
    }

    pub fn test_text_colors(&mut self) {
        // let mut canvas = self.led_matrix.offscreen_canvas();
        
        // Clear the canvas first
        self.led_canvas.clear();

        //hardcode font for testing
        let font = LedFont::new(
            Path::new("./4x6.bdf")
        ).unwrap();
        
        // Test different colors with simple text
        let colors = vec![
            ("RED", LedColor { red: 255, green: 0, blue: 0 }),
            ("GREEN", LedColor { red: 0, green: 255, blue: 0 }),
            ("BLUE", LedColor { red: 0, green: 0, blue: 255 }),
            ("YELLOW", LedColor { red: 255, green: 255, blue: 0 }),
            ("CYAN", LedColor { red: 0, green: 255, blue: 255 }),
            ("MAGENTA", LedColor { red: 255, green: 0, blue: 255 }),
            ("WHITE", LedColor { red: 255, green: 255, blue: 255 }),
        ];
        
        for (i, (text, color)) in colors.iter().enumerate() {
            let y_pos = 8 + (i as i32 * 2);
            debug!("Drawing '{}' in color {:?} at y={}", text, color, y_pos);
            self.led_canvas.draw_text(&font, text, 2, y_pos, color, 0, false);
        }
        
        // Also draw some colored lines for comparison
        for i in 0..7 {
            let y_pos = 8 + (i as i32 * 2) + 4;
            self.led_canvas.draw_line(80, y_pos, 120, y_pos, &colors[i].1);
        }
        
        // canvas = self.led_matrix.swap(canvas);
    }

    pub fn test_colors(&mut self) {
        for red in (0..255).step_by(16) {
            for green in (0..255).step_by(16) {
                for blue in (0..255).step_by(16) {
                    self.test_color(red, green, blue);
                    thread::sleep(time::Duration::from_secs(2));
                }
            }
        }
    }

    fn get_starting_position(&self, line_ref: &String) -> (i32, i32) {
        // Row Position is (font_height + 1) * (n_row + 1)
        //col Position is 2 for left and COL_WIDTH + 2 for right for some buffer

        let (col, row) = self.display_position_map.get(line_ref).unwrap_or(&(1, 2));
        let x = (COL_WIDTH * *col) + 2;
        let y = (FONT_HEIGHT + 1) * (*row + 2);
        debug!(
            "line_ref='{}', col={}, row={}, calculated=({},{})",
            line_ref, col, row, x, y
        );
        return (x, y);
    }

    pub fn write_times(&mut self) {
        // let mut canvas = self.led_matrix.offscreen_canvas();
        self.led_canvas.clear();
        let mut curr_row = FONT_HEIGHT;
        let mut curr_time = String::from("Now ");
        let now = Local::now();
        if now.month() == 2 && now.day() == 2 {
            curr_time.push_str("YOUR BIRTHDAY!");
        } else {
            curr_time.push_str(&now.format("%H:%M:%S").to_string());
        }
        debug!(
            "writing current time: {:?}, at position: {:?}",
            curr_time,
            (2, curr_row)
        );
        let color = LedColor {
            red: 255,
            green: 255,
            blue: 255,
        };
        self.led_canvas.draw_text(
            &self.font, &curr_time, 2, // little bit of buffer
            curr_row, &color, //&TOP_LINE_COLOR,
            0, false,
        );

        if let Some(request_time) = self.last_successful_request_time {
            let mut last_updated = String::from("As of ");
            last_updated.push_str(&request_time.format("%H:%M:%S").to_string());
            debug!(
                "writing last updated: {:?}, at position: {:?}",
                last_updated,
                (COL_WIDTH + 2, curr_row)
            );
            self.led_canvas.draw_text(
                &self.font,
                &last_updated,
                COL_WIDTH + 2,
                curr_row,
                &TOP_LINE_COLOR,
                0,
                false,
            );
        }

        let lines_to_write = self.get_bus_styled_lines();

        for (index, line) in lines_to_write.iter().enumerate() {
            let (mut col_pos, mut curr_row) = self.get_starting_position(&line.line[0].string);
            debug!("starting position: {:?}, {:?}", col_pos, curr_row);
            for line_str in &line.line {
                if line_str.is_line_ref {
                    debug!(
                        "writing line: {:?} at position: {:?}",
                        line_str.string,
                        (col_pos, curr_row)
                    );
                    self.led_canvas.draw_text(
                        &self.font,
                        &line_str.string,
                        col_pos,
                        curr_row,
                        &line_str.color,
                        0,
                        false,
                    );
                    // two spaces after the four spaces for the line ref
                    col_pos += (LINE_REF_N_CHARS as i32) * 4 + LINE_REF_BUFFER_COLS;
                } else {
                    // buffer two spaces for chars (e.g. 10 = 10 or 4 = ' 4')
                    let to_write: String = line_str.string.chars().take(TTA_N_CHARS).collect();
                    let to_write = format!("{:>width$}", to_write, width = TTA_N_CHARS);
                    debug!(
                        "to write: {:?} at position: {:?}",
                        to_write,
                        (col_pos, curr_row)
                    );
                    self.led_canvas.draw_text(
                        &self.font,
                        &to_write,
                        col_pos,
                        curr_row,
                        &line_str.color,
                        0,
                        false,
                    );
                    col_pos += 4 * (TTA_N_CHARS as i32); //4 * 2

                    // write the dot if we don't have a loc
                    if !line_str.has_loc {
                        self.led_canvas.set(
                            col_pos - 1,                // deal with kearning
                            curr_row - FONT_HEIGHT + 1, //top pixel row for curr row
                            &NO_LOC_COLOR,
                        );
                    }
                    col_pos += 2;
                    col_pos += TTA_BUFFER_COLS;
                }
            }
        }
        // canvas = self.led_matrix.swap(canvas);
    }

    // pub fn lines_to_write(self) -> Vec<String> {

    // }
    pub fn get_bus_styled_lines(&self) -> Vec<RGBDisplayLine> {
        let mut lines = Vec::<RGBDisplayLine>::new();
        let display_lines = match &self.display_lines {
            Some(dl) => dl,
            None => return lines,
        };

        let mut sorted_keys = Vec::new();
        for key in display_lines.keys() {
            sorted_keys.push(key.clone());
        }
        sorted_keys.sort();

        let longest_key = 4;

        for key in sorted_keys {
            let mut this_line = RGBDisplayLine::new();
            let first_mvj = &display_lines[&key][0];
            let line_ref = first_mvj.line_ref.clone();
            let line_ref: String = line_ref.chars().take(LINE_REF_N_CHARS).collect();
            let line_ref_padded = format!("{:<width$}", line_ref, width = LINE_REF_N_CHARS);

            this_line.line.push(LineString {
                string: line_ref_padded,
                color: LINE_NAME_COLOR,
                has_loc: false,
                is_line_ref: true,
            });

            for mvj in &display_lines[&key] {
                match mvj.time_to_arrival() {
                    Some(tta) => {
                        // rapid lines the line ref isn't the same
                        // as the key
                        if mvj.has_location() {
                            this_line.line.push(LineString {
                                string: tta.to_string(),
                                color: TTA_COLOR,
                                has_loc: true,
                                is_line_ref: false,
                            })
                        } else {
                            this_line.line.push(LineString {
                                string: tta.to_string(),
                                color: TTA_COLOR,
                                has_loc: false,
                                is_line_ref: false,
                            })
                        }
                    }
                    None => (),
                }
            }
            lines.push(this_line);
        }
        return lines;
    }
}
