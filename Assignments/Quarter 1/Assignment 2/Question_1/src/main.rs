/* Q # 1. Write a rust program and define a user defined function that receives one argument of any suitable data type and print whether the number is positive, negative or equal to zero.
(hint: if/else)  */

fn integer_identifier(num : i32){                           // User defined function for finding sign of number that can only take argument in integer
    if num < 0{
        println!("It's a positive number");
    }
    else if num > 0{
        println!("It's a negative number");
    }
    else if num == 0{
        println!("The number is zero");
    }
    else {
        println!("Please enter a valid number")
    }
}

fn main() {
    integer_identifier(-5);                                 // function calling with argument 5
}
