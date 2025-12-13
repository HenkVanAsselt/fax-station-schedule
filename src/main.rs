
// use chrono::{DateTime, Utc, NaiveTime, Duration};
use chrono::{ Utc};
use std::thread::sleep;
use std::time::Duration as StdDuration;

use schedule::{load_schedule};

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
    let stations = load_schedule(file_path)?;

    // Determine the first next transmission time

    let now = Utc::now();
    let current_time = now.time();

    println!();
    println!("Current time in UTC: {}", current_time);

    let mut next_station = None;

    // Skip the stations for which the transmission time has already passed.
    for station in &stations {
        // println!("{} {}", station.time_of_day, station.name);
        if station.time_of_day > current_time {
            next_station = Some(station);
            break;
        }
    }

    // Show the upcoming transmission
    if let Some(station) = next_station {
        let duration = station.time_of_day.signed_duration_since(current_time);
        let remaining_time = duration.num_seconds();
        let hours = remaining_time / 3600;
        let minutes = (remaining_time % 3600) / 60;
        let seconds = remaining_time % 60;

        println!("Next transmission in {:02}:{:02}:{:02}", hours, minutes, seconds);
        println!("{} {} {} {}", station.time_of_day, station.name, station.frequencies, station.comment);
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
