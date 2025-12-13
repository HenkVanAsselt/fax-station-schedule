// use std::env;
// use std::fs::File;
// use std::io::Read;
use csv::{ReaderBuilder, Trim};
use chrono::{NaiveTime};

#[derive(Debug)]
pub struct RadioStation {
    pub time_of_day: NaiveTime,
    pub name: String,
    pub frequencies: String,
    pub comment: String,
}

type Record = (String, String, String, String);

pub fn load_schedule(path: &str) -> Result<Vec<RadioStation>, Box<dyn std::error::Error>>
{

    // Open the CSV file which contains the transmission schedule.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(Trim::All)
        .from_path(path)
        .expect("Failed to open CSV file");

    let mut stations: Vec<RadioStation> = Vec::new();

    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: Record = result?;
        // println!("{:?}", record);

        let time_of_day = record.0.parse::<NaiveTime>().expect("Invalid time format");
        let name = record.1.to_string();
        let frequencies = record.2.to_string();
        let comment = record.3.to_string();

        stations.push(RadioStation {
            time_of_day,
            name,
            frequencies,
            comment,
        });
    }

    Ok(stations)
}