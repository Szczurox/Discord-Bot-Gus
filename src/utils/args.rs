use serenity::framework::standard::Args;

use crate::constants::time::DURATION_TIME;

// Get a duration from some command arguments
pub fn get_duration_from_args(args: &mut Args) -> Option<u32> {
    if !args.is_empty() {
        // Get mute duration time from arguments
        let duration_string: String = args.single::<String>().expect("Error getting duration string");
        // Get time unit (days, months, years, etc.)
        let time_unit: String = duration_string.chars().filter(|c| !c.is_digit(10)).collect();

        // Check if the duration is specified
        if DURATION_TIME.contains_key(&time_unit[..]) {
            // Get number of time units from the mute duration time string
            let duration_length_string: String = duration_string.chars().filter(|c| c.is_digit(10)).collect();
            let duration_length : u32 = duration_length_string.parse::<u32>().unwrap();

            return Some(DURATION_TIME.get(&time_unit[..]).unwrap() * duration_length);
        }
        else {
            args.rewind();
        }
    }

    None
}

// Get a reason from some command arguments
pub fn get_reason_from_args(args: &mut Args) -> String { 
    if args.is_empty() {
        return String::from("reason not provided");
    }
    String::from(args.rest())
}