use schedule::{
    get_next_transmission, get_next_transmission_index, load_transmission_schedule,
    print_next_transmission,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @todo: load commandline arguments

    // Load the transmission schedule from the CSV file
    let file_path = "./schedules/schedule.csv";
    let transmissions = load_transmission_schedule(file_path);

    // // Determine the first next transmission time
    // let current_time = Utc::now().time();

    let index = get_next_transmission_index(transmissions.clone());
    match index {
        Some(val) => println!("Next transmission index: {:?}", val),
        None => println!("No upcoming transmissions found."),
    }

    let next_transmission = get_next_transmission(transmissions.clone());
    match next_transmission {
        Some(val) => print_next_transmission(val, true),
        None => println!("No upcoming transmissions found."),
    }

    Ok(())
}
