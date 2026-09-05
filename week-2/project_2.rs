
struct  Okeke_and_Sons_Sales_Record {
	item: String,
	quantity: u32,
	amount: f64,

}

fn main() {
	let records = vec![
	    Okeke_and_Sons_Sales_Record { item: String::from("Toshiba"), quantity: 2, amount: 450_000.00},
	    Okeke_and_Sons_Sales_Record { item: String::from("Mac"), quantity: 1, amount: 1_500_000.00},
	    Okeke_and_Sons_Sales_Record { item: String::from("HP"), quantity: 3, amount: 750_000.00},
	    Okeke_and_Sons_Sales_Record { item: String::from("Dell"), quantity: 3, amount: 2_850_000.00},
	    Okeke_and_Sons_Sales_Record { item: String::from("Acer"), quantity: 1, amount: 250_000.00},

	    ];
	    let sum: f64 = records.iter().map(|r| r.amount).sum();
	    let average = sum / records.len() as f64;

	    println!("Sum of Okeke & Sons Sales Record: {:.2}", sum);
	    println!("Average of Okeke & Sons Sales Record: {:.2}", average);
}