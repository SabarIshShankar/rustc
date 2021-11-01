fn main(){
	println!("print file");

	//Formatting
	println!("Number: {}",1+4);

	//Positional
	println!("{1} {0} {2}","b","a","c" );

	//traits
	println!("{:b} {:x} {:o}",100,100,100)

	let name = "name";
	println!("{}", name);


}