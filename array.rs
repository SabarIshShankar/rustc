//array
//let name = [1, 3, 4];
fn main(){
	let arr = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());

	 let arr1: [i32;4] = [-1;4];
	 println!("{:?}", arr1);
	 println!("array size is :{}", arr1.len());
	 arraywithloop();
}

fn arraywithloop(){
	let arr2:[i32;4] = [1,2,3,4];
	println!("{:?}", arr2);
	println!("array size is: {}", arr2.len());

	for index in 0..4{
		println!("index is: {} & value is: {}", index, arr2[index]);
	}
}

