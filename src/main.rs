use schedule::{
    get_next_transmission, get_next_transmission_index, load_transmission_schedule,
    print_next_transmission,
};
use std::thread::sleep;
use std::time::Duration as StdDuration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @todo: load commandline arguments

    // Load the transmission schedule from the CSV file
    let file_path = "./schedules/schedule.csv";
    let countdown = true;

    // let transmissions = load_transmission_schedule(file_path);
    let transmissions = load_transmission_schedule(file_path).unwrap_or_else(|err| {
        eprintln!("Error loading transmission schedule: {}", err);
        Vec::new()
    });

    // Determine the first next transmission time (index in the vector)
    let index = get_next_transmission_index(transmissions.clone());
    match index {
        Some(val) => println!("Next transmission index: {:?}", val),
        None => println!("No upcoming transmissions found."),
    }

    // Get the next transmission
    let next_transmission = get_next_transmission(transmissions.clone());
    match next_transmission {
        Some(val) => print_next_transmission(val),
        None => println!("No upcoming transmissions found."),
    }

    // Print the next transmission every x seconds, but only if countdown is enabled.
    if countdown {
        let update_interval = 10;
        loop {
            sleep(StdDuration::from_secs(update_interval));
            let next_transmission = get_next_transmission(transmissions.clone());
            match next_transmission {
                Some(val) => print_next_transmission(val),
                None => println!("No upcoming transmissions found."),
            }
        }
    }



    Ok(())
}
