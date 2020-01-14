/*  Q # 3. Write a rust program and define a user defined function that receives a number and return the number itself and its square by using tuple. */

fn number_square (num : i32) -> (i32,i32){
    (num,i32::pow(num,2))
}

fn main() {
    let (num,square) = number_square(-5);
    println!("The number is {} & its square is {}",num,square);
}
