fn main() {
    println!("Hello, world!");
	let mut x = 5;
	println!("The value of x is: {}",x);
	x = 6;
	println!("The value of x is: {}",x);

	const MAX_POINTS: u32 = 10_0000;
	println!("The value of constant is : {}", MAX_POINTS);

	let spaces = "   ";
	let spaces = spaces.len();

	println!("Printing spaces : {}", spaces);

	let x = 2.0;		//f64
	let y: f32 = 3.0;	//f32

	println!("printing value of x : {}",x);
	println!("printing value of y : {}",y);
	
	let test_bool = true;
	
	// with explicit annotation
	let _f: bool = false;

	println!("{}",_f);
	println!("{}",test_bool);
}
