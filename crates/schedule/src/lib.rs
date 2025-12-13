// use std::env;
// use std::fs::File;
// use std::io::Read;
use csv::{ReaderBuilder, Trim};
use chrono::{NaiveTime, Utc};

#[derive(Debug)]
pub struct Transmission {
    pub time_of_day: NaiveTime,
    pub station_name: String,
    pub frequencies: String,
    pub comment: String,
}

type Record = (String, String, String, String);

pub fn load_transmission_schedule(path: &str) -> Result<Vec<Transmission>, Box<dyn std::error::Error>>
{

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
        let record: Record = result?;
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

    Ok(transmissions)
}

pub fn get_next_transmission(stations: Vec<Transmission>) -> Option<Transmission>
{
    let now = Utc::now().time();
    stations.into_iter().find(|station| station.time_of_day > now)
}