//statements
fn main(){
	let num:i32 = 5;
	if num > 0{
		println!("positive");
	}

	let code = "TN";
	let state = match code{
		"MH" => {println!("Found match for MH"); "Maharashtra"},
		"KL" => "Kerala",
		"TN" => "Tamil Nadu",
		_ => "Unknown"
	};
	println!("State name is {}", state);

	let num1 = 4;
	if num > 0{
		println!("positive");
	} else if num1 < 0{
		println!("negative");
	}
	else {
		println!("positive nor negative");
	}
}