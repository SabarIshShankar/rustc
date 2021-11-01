fn main(){
	println!("Hello");
	let string = "Rust01";
	println!("language is {}", string);

	let result = 10;
	let age:u32 = 20;
	let sum:i32 = 5-15;
	println!("{} {} {}", result, age, sum);

	let result = 10.00;
	let interest:f32 = 0.01;
	println!("{} {}", result, interest);

	//automatic type casting
	//let interest1:f32 = 8;
	//println!("interest is {}", interest1);
	
	//number separator
	let float_with_separator = 11_000.555_001;
	println!("float value {}", float_with_separator);

	let character:char = 'h';
	println!("character {}", character);
}