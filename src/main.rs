use std::{self, io, process::exit};

fn main() {
    let mut total_points: f64 = 0.0;
    println!("Type '420' to exit type '421' to erase.");

    loop {
        println!("Position to calculate points for position: ");

        let mut position = String::new();

        // Get position
        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        // Turn entry into an integer and a value between 1 and 100
        let position: f64 = match position.trim().parse() {
            Ok(num) => match num {
                420 => exit(0),
                421 => {
                    total_points = 0.0;
                    println!("Total point tally erased.");
                    continue;
                }
                1..=100 => f64::from(num),
                _ => {
                    println!("Invalid position. Make sure you enter a number between 1 and 101");
                    continue;
                }
            },
            Err(_) => {
                println!("Invalid position. Make sure you enter a number");
                continue;
            }
        };

        let points = 250.0f64 * f64::exp(f64::ln(250.0f64 / 15.0f64) / (1.0f64 - 100.0f64) * (position - 1.0f64));

        total_points = total_points + points;
        {
            println!(
                "Calculating points for position {} in a list of {}.",
                position, 100
            );
            println!("Points Worth: {:.2}", points);
            println!("Total points: {:.2} \n", total_points);
        }
    }
}
