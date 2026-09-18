use std::io;

fn read_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Nope, that's not a valid number, try again."),
        }
    }
}

fn main() {
    let a = read_f64("Enter a:");
    let b = read_f64("Enter b:");
    let c = read_f64("Enter c:");

    if a == 0.0 {
        println!("dont you know that a cannot be 0? This is not a quadratic equation...");
        return;
    }

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let sqrt_d = d.sqrt();
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
        println!("Two distinct real roots! YAY ;) x1 = {:2}, x2 = {:2}", x1, x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("Exactly one real root! YAY ;D  x = {}", x);
    } else {
        println!("No real roots, oh no! :( (discriminant = {}).", d);
    }
}