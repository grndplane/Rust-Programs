// By MTE 3-20-2025, my first Rust program. Cargo on...

use read_input::prelude::*;
use rand::prelude::*;
use ferris_print::ferrisprint;

fn get_random_number() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=10) // Generates a random number between 1 and 10 (inclusive)
}

fn main() {
    // (t) is a variable to store the number of tries it takes to guess the number.
    let mut t = 0;

    // (a) is a variable to store the number you need to guess.
    let a = get_random_number(); // see function get_random_number() above.

    // Clear terminal screen
    if let Err(e) = clearscreen::clear() {
        println!("Warning: Failed to clear screen: {}", e);
    }

    println!("*** GUESS THE NUMBER ***");
    loop {
        t += 1; // Count the number of tries it takes.

        // Get the number from the user.
            println!("Enter a number between 1 and 10.");
            let b = input::<u8>().get();

        // Validate input
        if !(1..=10).contains(&b) {
            println!("You entered a number below 1 or over 10, try again.");
            continue;
            
        }

        // Give user a programmed response.
        if b < a {
            println!("You need to go higher than {}.", b);
        } else if b > a {
            println!("You need to go lower than {}.", b);
        } else {
            // Clear terminal screen
            if let Err(e) = clearscreen::clear() {
            println!("Warning: Failed to clear screen: {}", e);
            }
            // Output for correct guess, with cute Ferris.
            ferrisprint!("You guessed the correct number {}. It only took {} tries", b, t);
            break; // Exit loop
        }
    }
}
