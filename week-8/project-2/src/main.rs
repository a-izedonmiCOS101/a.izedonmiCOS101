use std::io;

fn read_text() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // A vector of tuples: each tuple is (name, years of experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates are you interviewing?");
    let count: u32 = read_text().parse().expect("Invalid input");

    if count == 0 {
        println!("No candidates to check.");
        return;
    }

    for i in 0..count {
        println!("\nEnter name of candidate {}:", i + 1);
        let name = read_text();

        println!("Enter years of programming experience for {}:", name);
        let years: u32 = read_text().parse().expect("Invalid input");

        candidates.push((name, years));
    }

    // Walk the vector, remembering the position of the highest seen so far
    let mut best_index = 0;
    for i in 1..candidates.len() {
        if candidates[i].1 > candidates[best_index].1 {
            best_index = i;
        }
    }

    // Unpack the winning tuple into named variables
    let (name, years) = &candidates[best_index];
    println!("\nMost experienced: {} with {} years", name, years);
}