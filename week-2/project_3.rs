fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Value of TV after Depreciation over 3 years
	let a = p * (1.0 - (r/100.00)).powf(n);
	println!("Value of Ms. Akudo Ijezie's TV after 3 years depreciation is {:.2}", a);
}