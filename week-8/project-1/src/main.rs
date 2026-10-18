use std::io;

fn read_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

fn main() {
    // The levels, in order from lowest to highest
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // One vector per career track, in the SAME order as the levels
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Choose a career track:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    let track = read_number();

    let roles = match track {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid track.");
            return;
        }
    };

    println!("\nChoose your role:");
    for i in 0..roles.len() {
        println!("{}. {}", i + 1, roles[i]);
    }
    let choice = read_number();

    if choice == 0 {
        println!("Invalid role number.");
        return;
    }
    let index = choice - 1;

    match roles.get(index) {
        Some(role) => {
            if *role == "-" {
                println!("There is no such role in this track.");
            } else {
                println!("A {} holds position {}", role, levels[index]);
            }
        }
        None => println!("Invalid role number."),
    }
}