/* Q # 2​. Take any integer value in a variable then add ‘5’ to that then multiply it with ‘4’, and print on your screen/terminal. You are not allowed to change the variable name throughout the program i.e you have to do shadowing.  */

fn main() {                         // Main Function
    let x = 8;                      // Variable initialization
    let x = x + 5;                  // Variable shadowing
    let x = x * 4;                  // Variable shadowing
    println!("{}",x);               // Printing variable value
}
