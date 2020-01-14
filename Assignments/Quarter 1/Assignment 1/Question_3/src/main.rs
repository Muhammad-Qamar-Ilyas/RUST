/* Q # 3​. Write a Rust program to perform mathematical operations between two numbers. Declare two float variables and assign some hard-coded values to them. Then print the result of addition, subtraction, division, and multiplication in between these two variables.  */


fn main() {                                                     // Main function
    let b : f32 = 2.5;                                          // Hard coded float variable value
    let a : f32 = 5.0;                                          // Hard coded float variable value
    let mut add : f32 = 0.0;                                    // Mutable float variable value so it can be change 
    let mut subtract : f32 = 0.0;                               // Mutable float variable value so it can be change
    let mut multiply : f32 = 0.0;                               // Mutable float variable value so it can be change
    let mut divide : f32 = 0.0;                                 // Mutable float variable value so it can be change
    add = a+b;                                                  // Addition operation of two variables
    subtract = a- b;                                            // Subtraction operation of two variables
    multiply = a*b;                                             // Multiplication operation of two variables
    divide = a/b;                                               // Division operation of two variables
    println!("The two numbers are {} and {}\n",a,b);            // Printing two variable values
    println!("Addition = {}",add);                              // Printing add value
    println!("Subtraction = {}",subtract);                      // Printing subtract value
    println!("Multiplication = {}",multiply);                   // Printing multiply value
    println!("Division = {}",divide);                           // Printing divide value
}
