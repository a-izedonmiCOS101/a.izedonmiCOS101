use std::io;

fn main() 
{
    let mut input = String::new();

    println!("\nEnter Your height (cm):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person");
    }
    else if height >170.00 && height <= 195.0
    {
        println!("If you're not playing basketball or modelling what're you doing?");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You're short o");
    }
    else {
        println!("Abnormal height"); 
    }

}
