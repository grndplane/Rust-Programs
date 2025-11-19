use jiff::Zoned;

fn main() -> Result<(), jiff::Error> {
    let now = Zoned::now(); // Get the current time
     let hour = now.hour();              // Example: 1 (12-hour format)
    let minute = now.minute();          // Example: 5
    let am_pm = if now.hour() < 12 { "AM" } else { "PM" }; // Determine AM/PM
    let hour = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour }; // Convert to 12-hour format
    let day: i8 = now.day(); // Get the day of the month as an integer
    let day_of_week= now.weekday(); // Get the day of the week
    let monthnumber = now.month();     // Example: 4 (for April)
       
    // Shorten day_of_week to the first 3 characters
    let day_of_week_short = &format!("{:#?}", day_of_week)[..3];

    let month = match monthnumber {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    };  

    println!("{}, {} {:02},{:2}:{:02} {}", day_of_week_short, month, day, hour, minute, am_pm);
       
    Ok(())
}
