fn main(){
	let tuple:(i32, f64, u8) = (-325, 4.9, 22);
	println!("{:?}", tuple);

	println!("{}", tuple.1);
	let b:(i32, bool, f64) = (110, true, 10.9);
	print(b);
	let b1: (i32, bool, f64) = (30, true, 9.9);
	print1(b);
}

fn print(x: (i32, bool, f64)){
	println!("{:?}", x)
}

//destructing

fn print1(x: (i32, bool, f64)){
	println!("Inside print method");
	let (age, is_male, cgpa) = x;
//assigns to distinct variables
	println!("{}, {}, {}", age, is_male, cgpa);
}