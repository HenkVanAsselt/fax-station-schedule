// use std::env;
// use std::fs::File;
// use std::io::Read;
use std::thread::sleep;
use csv::{ReaderBuilder, Trim};
use chrono::{NaiveTime, Utc};
use std::time::Duration as StdDuration;

#[derive(Debug)]
pub struct Transmission {
    pub time_of_day: NaiveTime,
    pub station_name: String,
    pub frequencies: String,
    pub comment: String,
}

type Record = (String, String, String, String);

// fn load_csv(path: &str) -> Vec<Vec<String>> {
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(true)
//         .from_path(path)
//         .expect("Cannot open CSV file");
//
//     let mut rows = Vec::new();
//     for result in rdr.records() {
//         let record = result.expect("Invalid record");
//         rows.push(record.iter().map(|s| s.to_string()).collect());
//     }
//     rows
// }

// pub fn load_transmission_schedule(path: &str) -> Result<Vec<Transmission>, Box<dyn std::error::Error>>
pub fn load_transmission_schedule(path: &str) -> Vec<Transmission>
{
   
    ///
    /// Example of the contents of the CSV file:
    ///
    /// Time, Station, Frequencies, Comments
    /// 00:00, GYA Northwood, 2618.5 kHz / 4610 kHz / 8040 kHz, 18Z SURFACE ANALYSIS
    /// 00:12, GYA Northwood, 2618.5 kHz / 4610 kHz / 8040 kHz, 18Z SURFACE PROGNOSIS (24HR)
    /// 04:30, DDH3/DDK6, 3855 kHz / 7880 kHz / 13882.5 kHz, SURFACE ANALYSIS NORTH ATLANTIC / EUROPE
    /// 04:36, GYA Northwood, 2618.5 kHz / 4610 kHz / 8040 kHz, 00Z SURFACE PROGNOSIS (24HR)
    /// 05:12, DDH3/DDK6, 3855 kHz / 7880 kHz / 13882.5 kHz, 36HR FORECAST SURFACE PRESSURE
    ///
    
    // Open the CSV file which contains the transmission schedule. 
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(Trim::All)
        .from_path(path)
        .expect("Failed to open CSV file");

    // Create a vector to transmissions, found in the CSV file.
    let mut transmissions: Vec<Transmission> = Vec::new();

    for result in reader.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: Record = result.expect("Error reading record");
        // println!("{:?}", record);

        let time_of_day = record.0.parse::<NaiveTime>().expect("Invalid time format");
        let name = record.1.to_string();
        let frequencies = record.2.to_string();
        let comment = record.3.to_string();

        transmissions.push(Transmission {
            time_of_day,
            station_name: name,
            frequencies,
            comment,
        });
    }

    // Ok(transmissions)
    transmissions
}

pub fn get_next_transmission(transmissions: Vec<Transmission>) -> Option<Transmission>
{
    let now = Utc::now().time();
    transmissions.into_iter().find(|station| station.time_of_day > now)
}

pub fn print_next_transmission(transmission: Transmission, countdown: bool)
{
    // Show the upcoming transmission

    let current_time = Utc::now().time();

    let duration = transmission.time_of_day.signed_duration_since(current_time);
    let remaining_time = duration.num_seconds();
    let hours = remaining_time / 3600;
    let minutes = (remaining_time % 3600) / 60;
    let seconds = remaining_time % 60;

    println!("Next transmission in {:02}:{:02}:{:02}", hours, minutes, seconds);
    println!("{} {} {} {}", transmission.time_of_day, transmission.station_name, transmission.frequencies, transmission.comment);
    println!();

    if countdown {
        print_countdowntimer(remaining_time);
    }
}

pub fn print_countdowntimer(remaining_time: i64)
{
    let mut remaining = remaining_time;
    while remaining > 0 {
        let hours = remaining_time / 3600;
        let minutes = (remaining_time % 3600) / 60;
        let seconds = remaining_time % 60;
        println!("Time remaining: {:02}:{:02}:{:02}", hours, minutes, seconds);
        sleep(StdDuration::from_secs(1));
        remaining -= 1;
    }
}


