use chrono::Local;
use clap::Parser;
use reqwest::Client;
use std::collections::HashMap;
use std::io::{self, Write};
use std::{thread, time};
use transit_rust::display_board::DisplayBoard;
use transit_rust::stop_monitor::{LineStop, MonitoredVehicleJourney, get_stops};

use log::debug;
use rpi_led_matrix::{LedColor, LedFont, LedMatrix, LedMatrixOptions};
use std::fs;
use std::path::{Path, PathBuf};
use yaml_rust2::{Yaml, YamlLoader};

use transit_rust::config::Config;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "./etc/config.yml")]
    config_path: PathBuf,

    #[arg(short, long)]
    run_color_test: bool,

    #[arg(short, long)]
    test_write: bool,

    #[arg(short, long)]
    x_test_text_colors: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    debug!("Starting transit_rust");

    let args = Args::parse();
    let config_string = fs::read_to_string(args.config_path)?;
    let config_yaml = &YamlLoader::load_from_str(config_string.as_str()).unwrap()[0];
    let config = Config::new(config_yaml);

    let client = Client::new();

    debug!("creating display board...");

    let font_path = Path::new(config.display_board_config().font_file());
    debug!("font path: {:?}", font_path);
    if !font_path.exists() {
        panic!("font file doesn't exist");
    }
    let mut display_board = DisplayBoard::new(
        config.display_board_config().rows(),
        config.display_board_config().cols(),
        config.display_board_config().chained(),
        font_path,
        config.display_board_config().line_ref_to_display_position(),
    )
    .unwrap();
    debug!("Created display board");

    if args.run_color_test {
        run_color_test(&mut display_board);
        return Ok(());
    }
    if args.test_write {
        display_board.test_write();
        thread::sleep(time::Duration::from_secs(60));
        return Ok(());
    }
    if args.x_test_text_colors {
        display_board.test_text_colors();
        thread::sleep(time::Duration::from_secs(60));
        return Ok(());
    }

    let stops_to_monitor = config.stops_to_monitor();
    debug!("Stops to monitor: {:?}", stops_to_monitor);
    debug!("Starting update loop");
    loop {
        update_display_board(
            &mut display_board,
            &client,
            config.rapid_line_to_parent_line_map(),
            stops_to_monitor,
        )
        .await;
        display_board.write_times();
        if display_board.last_request_successful {
            thread::sleep(time::Duration::from_secs(30));
        } else {
            thread::sleep(time::Duration::from_secs(2));
        }
    }
    Ok(())
}

fn run_color_test(display_board: &mut DisplayBoard) {
    loop {
        println!("RGB Color Input Program");
        println!("Please enter values between 0-255 for each color component");
        println!("If red is less than 0 we break this loop");
        let red = read_i32_input("Red: ");
        let green = read_i32_input("Green: ");
        let blue = read_i32_input("Blue: ");
        if red < 0 {
            break;
        }
        display_board.test_color(red as u8, green as u8, blue as u8);
    }
}

fn read_i32_input(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                if num >= 0 && num <= 255 {
                    return num;
                } else {
                    println!("Value must be between 0 and 255. Please try again.");
                }
            }
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

async fn update_display_board(
    display_board: &mut DisplayBoard,
    client: &Client,
    rapid_line_to_parent_map: &HashMap<String, String>,
    stops_to_monitor: &Vec<String>,
) {
    if let Ok(stops) = get_stops(client, stops_to_monitor).await {
        if let Ok(display_lines) = get_display_lines(stops, rapid_line_to_parent_map, true) {
            debug!("Received lines to display");
            (*display_board).display_lines = Some(display_lines);
            (*display_board).last_successful_request_time = Some(Local::now());
            (*display_board).last_request_successful = true;
        } else {
            debug!("Failed to get display lines");
            (*display_board).last_request_successful = false;
        }
    } else {
        (*display_board).last_request_successful = false;
    }
}

fn get_display_lines(
    stops: HashMap<LineStop, Vec<MonitoredVehicleJourney>>,
    rapid_line_to_parent_map: &HashMap<String, String>,
    use_line_to_parent_map: bool,
) -> Result<HashMap<String, Vec<MonitoredVehicleJourney>>, reqwest::Error> {
    const DEFAULT_TIME_TO_ARRIVAL: i64 = 999;
    let mut display: HashMap<String, Vec<MonitoredVehicleJourney>> = HashMap::new();

    for (line_stop, value) in stops.into_iter() {
        let parent_line = if use_line_to_parent_map {
            match rapid_line_to_parent_map.get(line_stop.line_ref.as_str()) {
                Some(parent_line) => parent_line.clone(),
                None => line_stop.screen_display(),
            }
        } else {
            line_stop.screen_display()
        };

        // Add new time to arrivals or create a new entry in display lines
        for mvj in value {
            if mvj.time_to_arrival().is_some() {
                display
                    .entry(parent_line.clone())
                    .or_insert_with(Vec::new)
                    .push(mvj);
            }
        }
    }

    // Sort the values by time to arrival
    for value in display.values_mut() {
        value.sort_by_key(|a| a.time_to_arrival().unwrap_or(DEFAULT_TIME_TO_ARRIVAL));
    }

    Ok(display)
}
