
// use chrono::{DateTime, Utc, NaiveTime, Duration};
use chrono::{ Utc};
use std::thread::sleep;
use std::time::Duration as StdDuration;

use schedule::{load_transmission_schedule, get_next_transmission};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get commandline argument, which should be the path to the CSV file
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <path_to_csv>", args[0]);
    //     return;
    // }
    // let file_path = &args[1];

    // Load the transmission schedule from the CSV file
    let file_path = "./schedules/schedule.csv";
    let transmissions = load_transmission_schedule(file_path)?;

    // Determine the first next transmission time
    let current_time = Utc::now().time();

    let next_transmission = get_next_transmission(transmissions);

    // Show the upcoming transmission
    if let Some(transmission) = next_transmission {
        let duration = transmission.time_of_day.signed_duration_since(current_time);
        let remaining_time = duration.num_seconds();
        let hours = remaining_time / 3600;
        let minutes = (remaining_time % 3600) / 60;
        let seconds = remaining_time % 60;

        println!("Next transmission in {:02}:{:02}:{:02}", hours, minutes, seconds);
        println!("{} {} {} {}", transmission.time_of_day, transmission.station_name, transmission.frequencies, transmission.comment);
        println!();

        // Show a countdown timer
        let mut remaining_time = duration.num_seconds();
        while remaining_time > 0 {
            let hours = remaining_time / 3600;
            let minutes = (remaining_time % 3600) / 60;
            let seconds = remaining_time % 60;
            println!("Time remaining: {:02}:{:02}:{:02}", hours, minutes, seconds);
            sleep(StdDuration::from_secs(1));
            remaining_time -= 1;
        }

    } else {
        println!("No upcoming transmissions found.");
    }

    Ok(())

}
