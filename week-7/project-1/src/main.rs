use std::io;

// returns a value (Practice 3)
fn get_pi() -> f64 {
    let a: f64 = 22.0;
    let b: f64 = 7.0;
    return a / b;
}

// reads one number from the keyboard
fn read_number(message: &str) -> f64 {
    let mut input = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: f64 = input.trim().parse().expect("Invalid input");
    return number;
}

// reads the menu choice, -1 if it is not a number
fn read_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().parse() {
        Ok(number) => number,
        Err(_) => -1,
    }
}

// one function per shape
fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    return height / 2.0 * (base1 + base2);
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter diagonal 1:");
    let diagonal2 = read_number("Enter diagonal 2:");
    return 0.5 * diagonal1 * diagonal2;
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    return base * altitude;
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side:");
    return 6.0 * side * side;
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    return get_pi() * radius * radius * height;
}

// pass by value: the function gets a copy of the array
fn show_menu(shapes: [&str; 5]) {
    println!("\n=== Shape Calculator ===");
    for index in 0..5 {
        println!("{}. {}", index + 1, shapes[index]);
    }
    println!("0. Quit");
}

// pass by value again: copies of the name and the answer
fn show_result(name: &str, answer: f64) {
    println!("The {} is {:.2}", name, answer);
}

// pass by reference: changes the caller's counter (Practice 6)
fn count_calculation(counter: &mut i32) {
    *counter = *counter + 1;
}

fn main() {
    let shapes: [&str; 5] = [
        "Trapezium area",
        "Rhombus area",
        "Parallelogram area",
        "Cube surface area",
        "Cylinder volume",
    ];
    let mut results: [f64; 5] = [0.0; 5];
    let mut counter: i32 = 0;

    loop {
        show_menu(shapes);
        println!("Choose an option:");
        let choice = read_choice();

        if choice == 0 {
            break;
        }
        if choice < 1 || choice > 5 {
            println!("Invalid choice, try again.");
            continue;
        }

        let answer = match choice {
            1 => trapezium_area(),
            2 => rhombus_area(),
            3 => parallelogram_area(),
            4 => cube_surface_area(),
            _ => cylinder_volume(),
        };

        let index = (choice - 1) as usize;
        results[index] = answer;
        show_result(shapes[index], answer);
        count_calculation(&mut counter);
    }

    println!("\nCalculations done: {}", counter);
    println!("Results array is {:?}", results);

    // loop by index (Practice 8)
    for index in 0..5 {
        println!("{} -> {}", shapes[index], results[index]);
    }

    // loop with iter() (Practice 9)
    let mut total = 0.0;
    for value in results.iter() {
        total = total + *value;
    }
    println!("Sum of all results is {:.2}", total);
}