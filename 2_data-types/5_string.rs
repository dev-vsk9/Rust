fn main(){
    // string slice
    let slice: &str = "Rust";
    // conversion to string
    let mut owned = slice.to_string(); 

    // ADD
    owned.push_str(" Architect");


    let final_signal = format!("{} Status: Active", owned);
    
    println!("Broadcast: {}", final_signal);


}