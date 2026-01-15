// immutable- global -SCREAMING_SNAKE_CASE- fixed
const MAX_SPEED: u32 = 9000;

fn main(){
    // explicit - runtime
    let age: u32 = 30;

    // inferred as i32
    let power = 100;

    //discard
    let _multiplication = 5*5 ;

    println!("System speed: {}", MAX_SPEED);
    println!("age:{},power:{}",age,power)
}