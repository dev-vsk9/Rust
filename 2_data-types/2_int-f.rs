fn main(){
    //unsigned
    let max_u8: u8 = 255;

    //signed 
    let min_i8: i8 = -128;

    let pi: f32 = 3.14159;

    println!("unsigned max:{}, Signed min:{}",max_u8,min_i8);
    println!("float:{}",pi);

    

    let handled_overflow = max_u8.wrapping_add(1); 
    println!("Overflow Handled (Wrapped): {}", handled_overflow);

    let overflow = max_u8 + 1; 
    println!("overflow:{}",overflow);



}