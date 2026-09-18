use std::io;

fn read_age() -> u32 {
    loop {
        println!("Enter the employee's age:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(age) => return age,
            Err(_) => println!("Please type a whole number, as I assume you know what that is."),
        }
    }
}

fn read_experienced() -> bool {
    loop {
        println!("Is the employee experienced? (yes/no):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please type yes or no."),
        }
    }
}

fn main() {
    let experienced = read_experienced();
    let age = read_age();

    let incentive: u32 = if !experienced {
        100_000
    } else if age >= 40 {
        1_560_000
    } else if age >= 30 {
        1_480_000
    } else if age < 28 {
        1_300_000
    } else {
        println!("No incentive rule is defined for ages 28 and 29.");
        return;
    };

    println!("Annual incentive: N{}", incentive);
}