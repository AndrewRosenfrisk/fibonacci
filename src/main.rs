use bnum::types::U8192;
use std::{io::stdin, thread, time::Duration};

fn main() {
    'main: loop {
        println!("Enter the Nth Fibonacci number you wish to\ncalculate (e.g., 5, 50, 1000, 9999), or [Q] to quit:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input.");
        input = input.trim().to_uppercase();

        if input == "Q" {
            println!("Goodbye.");
            break 'main;
        } else if input.parse::<usize>().is_err() || input.parse::<usize>().unwrap() > 12000 {
            println!("Invalid input - enter a positive, whole number < 12000. Requests >= 11803 will still error for demonstration.");
            continue 'main;
        } else {
            let calculate_nth = input.parse::<usize>().expect("Error checking input bound.");

            let mut last: U8192 = bnum::BUint::ZERO;
            let mut next = bnum::BUint::ONE;
            let mut calculated_numbers = vec![];

            for nth in 0..calculate_nth {
                let temp = next;

                calculated_numbers.push(last);
                next = next.saturating_add(last);
                if next == last {
                    println!("Saturated memory at {}th number in sequence. Printing partial sequence to this point...", nth + 1);
                    thread::sleep(Duration::from_secs(2));
                    break;
                }
                last = temp;
            }

            let display_string = calculated_numbers
                .iter()
                .map(|n| n.to_string())
                .enumerate()
                .map(|pair| "(".to_string() + &(pair.0 + 1).to_string() + ") " + &pair.1)
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "The Fibonacci sequence the the {}th number is: \n {}",
                calculate_nth, display_string
            );
        }
    }
}
