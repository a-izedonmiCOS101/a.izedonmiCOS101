use std::fs::File;
use std::io::Write; 
fn main () {
    let students = vec!{
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("AJ Baby", "EEE11020202", "Electrical Engineering", 200),
        ("Bianca Edemo", "MEE10202001", "Mechanical Engineering", 100),

    };

    
    for student in &students {
                println!("{} | {} | {} | {}", student.0, student.1, student.2, student.3);

    }

    

let mut file = File::create("students.txt").expect("Couldn't do that, sorry!");

for student in &students {
    let details = format!("{} | {} | {} | {}\n", student.0, student.1, student.2, student.3);

file.write_all(details.as_bytes()).expect("Couldn't write to file");
}
println!("Student details saved successfully!");
}
