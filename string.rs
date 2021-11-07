//strings in rust
fn main(){
	let company:&'static str = "Rust tutorial";
	let location:&'static str = "location";
	println!("{} {}", company, location);

	let empty_string = String::new();
	println!("length is {}", empty_string.len());

	let mut z = String::new();
	z.push_str("hello");
	println!("{}", z);

	let name1 = "Hello tutorialspoint".to_string();
	println!("{}", name1);
	let name2 = name1.replace("Hello", "Howdy");
	println!("{}", name2);

	let fullname = "Tutorials Point \r\n";
	println!("Before trim");
	println!("length is {}", fullname.len());
	println!();
	println!("After trim");
	println!("length is {}", fullname.trim().len());

	let msg = "Tutorialpoints has good tutorials".to_string();
	let mut i = 1;
	for token in msg.split_whitespace(){
		println!("token {} {}", i, token);
		i += 1;
	}

	let fullname="John,Wick,Reeves";
	for token in fullname.split(","){
		println!("token {}", token);
	}

	println!("\n");
	let tokens:Vec<&str> = fullname.split(",").collect();
	println!("{} {} {}", tokens[0], tokens[1], tokens[2]);

	let n11 = "Tutorials".to_string();
	for n1 in n11.chars(){
		println!("{}", name1);
	}
}

