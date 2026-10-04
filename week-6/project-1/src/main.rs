use std::io;

fn ask(question: &str) -> String {
    println!("{}", question);
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    text.trim().to_uppercase()
}

fn main() {
    println!("--- MAMA PAU'S KITCHEN ---");
    println!("P  Poundo Yam / Edinkaiko Soup  N3,200");
    println!("F  Fried Rice & Chicken         N3,000");
    println!("A  Amala & Ewedu Soup           N2,500");
    println!("E  Eba & Egusi Soup             N2,000");
    println!("W  White Rice & Stew            N2,500");

    let price = match ask("Pick a letter:").as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => { println!("That is not on the menu!"); return; }
    };

    let quantity: u32 = ask("How many plates?").parse().unwrap_or(1);
    let total = price * quantity;
    let discount = if total > 10000 { total * 5 / 100 } else { 0 };
    println!("Total: N{}\nDiscount: N{}\nYou pay: N{}", total, discount, total - discount);
}