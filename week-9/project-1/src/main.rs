use std::io::Write;
fn main() {
    let drinkables = "LAGER
    33 Export
    Desperados
    Goldberg
    Gulder
    Heineken
    Star

    STOUT
    Legend
    Turbo King
    Williams

    ALL THINGS MALT
    Maltina
    Amstel Malta
    Malta Gold 
    Fayrouz";

    let mut file = std::fs::File::create("drinkables.txt").expect("Couldn't create file");
    file.write_all(drinkables.as_bytes()).expect("Couldn't write to file");
    println!("Drinks saved successfully!");
}