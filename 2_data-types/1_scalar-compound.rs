fn main(){
    //scalar
    //int
    let age: i32 =30;
    //boolean
    let booolean: bool = true;

    //compound
    //tuple
    let coordinates: (i32, i32) = (10, 20); 
    // array
    let vault_row: [u8; 3] = [1, 2, 3]; 

  println!("Scalar: age = {}, booolean = {}", age, booolean);
  println!("Compound Tuple: x = {}, y = {}", coordinates.0, coordinates.1);
  println!("Compound Array: 1st vault = {}, 3rdvault = {}", vault_row[0],vault_row[2]);

}