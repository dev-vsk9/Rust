fn main(){
    //array
    let arr:[i32;5] =[1,3,4,5,6];
    let large_arr = [1; 1000];


    //acccess
    let third_element = arr[2]; 
    println!("Array 3rd Element: {}", third_element);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    println!("Tuple 2nd Element: {}", tup.1);

     // deconstruciton
    let (gold, silver, _bronze) = tup; 
    println!("Deconstructed Tuple: gold = {}, silver = {}", gold, silver);

    println!("_large_arr (first 10): {:?}", &large_arr[..10]);

}