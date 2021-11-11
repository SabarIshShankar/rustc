//function
fn main(){
	println!("pi value is {}", get_pi());
	let no:i32 = 5;
	mutate_no_to_zero(no);
	println!(
		"The value of no is: {}", no
	);
	let name:String = String::from ("TutorialsPoint");
	display(name);
}
fn get_pi()->f64{
	22.0/7.0
}
fn mutate_no_to_zero(mut param_no:i32){
	param_no = param_no*0;
	println!("param_no value is {}", param_no);
}
fn display(param_name: String){
	println!("param_name value is {}", param_name);
}