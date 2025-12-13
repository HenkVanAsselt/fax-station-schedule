
// use chrono::{DateTime, Utc, NaiveTime, Duration};
use chrono::{ Utc};
use std::thread::sleep;
use std::time::Duration as StdDuration;

use schedule::{load_transmission_schedule, get_next_transmission, print_next_transmission};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // @todo: load commandline arguments

    // Load the transmission schedule from the CSV file
    let file_path = "./schedules/schedule.csv";
    let transmissions = load_transmission_schedule(file_path)?;

    // Determine the first next transmission time
    let current_time = Utc::now().time();

    let next_transmission = get_next_transmission(transmissions);
    match next_transmission {
        Some(val) => print_next_transmission(val, true),
        None => println!("No upcoming transmissions found."),
    }
    
    Ok(())

}
