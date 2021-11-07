fn main(){
	for x in 1..11{
		if x==5{
			continue;
		}
		println!("{}", x);
	}

	let mut y = 0;
	while y < 10{
		y+=1;
		println!("inside loop y value is {}", y);
	}
	println!("outside loop y value is {}", y);
}