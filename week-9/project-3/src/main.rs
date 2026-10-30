use std::fs::File;
use std::io::Write;

fn main()  {
    let commissioners = vec![
    "Aigobun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etiye",
    ];

    let ministries = vec![
    "Internal Affairs",
    "Justice",
    "Defense",
    "Power & Steel",
    "Petroleum",
    ];

    let zones = vec![
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",
    ];

    let mut file = File::create("efcc.txt").expect("Could not create file");

    for i in 0..commissioners.len() {
        println!("{} | {} | {}", commissioners[i], ministries[i], zones[i] );

        writeln!(file, "{} | {} | {}", commissioners[i], ministries[i], zones[i]).expect("could not write to file");

    }

    println!("EFCC data merged successfully!");
}